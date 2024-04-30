// cargo run -- https://www.cloudflare.com/rate-limit-test/

mod options;

use std::sync::{mpsc, Arc, Mutex};
use std::time::*;
use std::{env, thread};
use clap::Parser;
use crate::options::Cli;


fn main() {
    let arguments = Cli::parse();

    let mut running: bool = true;
    let mut method= "get".to_string();
    let mut post_d= String::new();
    // get arguments
    let url = arguments.url;
    let delay = arguments.delay;
    let post_data = arguments.post_data;

    if let Some(post_data) = post_data {
        method = "post".to_string();
        post_d = post_data
    }

    // let now = chrono::Local::now();
    let now = SystemTime::now();
    let count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    loop {
        let ctx = tx.clone();
        let burl = url.clone();
        let b_count = count.clone();
        let b_method = method.clone();
        let b_post_d = post_d.clone();
        thread::spawn(move || {
            let mut n_count = b_count.lock().unwrap();
            *n_count += 1;
            let cur_count = *n_count;

            let are_we_throttled = SystemTime::now();
            let res = send_request(burl.as_str().clone(), b_method, b_post_d);
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

fn send_request(url: &str, method: String, post_data: String) -> String {
    match method.as_str() {
        "post" => {
            let client = reqwest::Client::new();
            let tk = tokio::runtime::Runtime::new();
            let req = tk.unwrap().block_on(client.post(url)
                .body(post_data)
                .send());
            // println!("{:?}", req);
            // req.unwrap().status().to_string()
            // if req.is_err() {
            if let Ok(req) = req {
                req.status().to_string()
            } else {
                return match req.err().unwrap().status() {
                    None => "Request Failed".to_string(),
                    Some(status) => status.to_string(),
                }
            }
        }
        &_ => {
            let req = reqwest::blocking::get(url);
            // println!("{:?}", req);
            // req.unwrap().status().to_string()
            // if req.is_err() {
            if let Ok(req) = req {
                req.status().to_string()
            } else {
                return match req.err().unwrap().status() {
                    None => "Request Failed".to_string(),
                    Some(status) => status.to_string(),
                }
            }
        }
    }
}
