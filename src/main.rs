mod options;

use crate::options::Cli;
use clap::Parser;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::*;
use reqwest::header::{HeaderName, HeaderValue};

macro_rules! thread_func {
    ($a:expr, $sm:expr, $def_par:expr) => {
        let mut handles = vec![];
        for _i in 0..$def_par {
            let sm = $sm.clone();
            let tk = tokio::runtime::Runtime::new();
            let handle = thread::spawn(move || tk.unwrap().block_on($a(sm)));
            handles.push(handle);
            // let _ = tokio::time::sleep(Duration::new(0, 200000000)).await;
            // let _ = tokio::time::sleep(Duration::new(1, 0)).await;
            // let _ = tokio::time::sleep(Duration::new(2, 0)).await;
        }

        // let _ = tokio::time::sleep(Duration::new(2, 0)).await;

        for handle in handles {
            // handle.join().unwrap()
            if let Err(_) = handle.join() {
                println!("WARNING: could not join on handle")
            }
        }
    };
}

fn main() {
    let arguments = Cli::parse();

    let mut running: bool = true;
    let mut method = "get".to_string();
    let mut post_d = String::new();
    let mut header_d: Vec<String> = vec![];
    // get arguments
    let url = arguments.url;
    let delay = arguments.delay;
    let post_data = arguments.post_data;
    let header_data = arguments.array_headers;
    let method_data = arguments.method;

    if let Some(post_data) = post_data {
        post_d = post_data
    }

    if let Some(headers) = header_data {
        let a = headers.split(',').map(|e| {e.to_string()}).collect::<Vec<String>>();
        header_d.clone_from(&a);
    }

    if let Some(method_d) = method_data {
        method = method_d;
    }


    // let now = chrono::Local::now();
    let now = SystemTime::now();
    let count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));



    let burl = url.clone();
    let b_count = count.clone();
    let b_method = method.clone();
    let b_post_d = post_d.clone();
    let b_header_d = header_d.clone();

    start_loop(burl, b_count, b_method, b_post_d, b_header_d, now, running, delay)
}

fn start_loop(burl: String, b_count: Arc<Mutex<u64>>, b_method: String, b_post_d: String, b_header_d: Vec<String>, now: SystemTime, run: bool, delay: u64) {
    let (tx, rx) = mpsc::channel();
    let mut running = run;
    loop {

        let ctx = tx.clone();
        thread::spawn(move || {
            let mut n_count = b_count.lock().unwrap();
            *n_count += 1;
            let cur_count = *n_count;

            let are_we_throttled = SystemTime::now();
            let res = send_request(burl.as_str(), b_method.clone(), b_post_d.clone(), b_header_d.clone());
            if are_we_throttled.elapsed().unwrap().as_millis() > 500
                && are_we_throttled.elapsed().unwrap().as_millis() < 826
            {
                println!(
                    "Slow... {}ms",
                    are_we_throttled.elapsed().unwrap().as_millis()
                );
            }
            if are_we_throttled.elapsed().unwrap().as_millis() > 826 {
                println!(
                    "Throttled... {}ms",
                    are_we_throttled.elapsed().unwrap().as_millis()
                );
            }

            let msg = format!(
                "{}, Time elapsed: {}, Requests sent: {}",
                res,
                now.elapsed().unwrap().as_secs(),
                cur_count
            );
            ctx.send(msg).unwrap();

            if !res.contains("200") {
                running = false;
            }

            if !running {
                println!("Requests blocked after: {}ms", cur_count);
            }
        });

        if delay > 100 {
            thread::sleep(Duration::new(0, delay as u32 * 1_000_000));
        } else {
            thread::sleep(Duration::new(delay, 0));
        }

        let rc = rx.recv().unwrap();
        println!("{}", rc);
    }
}

macro_rules! get_status {
    ($a:expr) => {
        if let Ok(req) = $a {
            req.status().to_string()
        } else {
            match $a.err().unwrap().status() {
                None => "Request Failed".to_string(),
                Some(status) => status.to_string(),
            }
        }
    };
}

fn send_request(url: &str, method: String, post_data: String, header_d: Vec<String>) -> String {
    let mut headers = reqwest::header::HeaderMap::new();
    if !header_d.is_empty() {
        header_d.iter().for_each(|e| {
            let split = e.split(':').map(|a| {a.trim().to_string()}).collect::<Vec<String>>();
            let _ = headers.try_append(HeaderName::from_bytes(split.get(0).unwrap().as_bytes()).unwrap(), HeaderValue::from_bytes(split.get(1).unwrap().as_bytes()).unwrap());
        });
        // println!("{:?}", headers);
    }
    match method.as_str() {
        "post" => {
            println!("Sending POST:");
            let client = reqwest::Client::new();
            let tk = tokio::runtime::Runtime::new();
            let req = tk.unwrap()
                .block_on(
                    client.post(url)
                        .headers(headers)
                        .send()
                );
            get_status!(req)
        }
        "get" => {
            println!("Sending GET:");
            let client = reqwest::Client::new();
            let tk = tokio::runtime::Runtime::new();
            let req = tk.unwrap()
                .block_on(
                    client.get(url)
                        .headers(headers)
                        .send()
                );
            get_status!(req)
        }
        &_ => {
            println!("Sending GET:");
            let client = reqwest::Client::new();
            let tk = tokio::runtime::Runtime::new();
            let req = tk.unwrap()
                .block_on(
                    client.get(url)
                        .headers(headers)
                        .send()
                );
            get_status!(req)
        }
    }
}
