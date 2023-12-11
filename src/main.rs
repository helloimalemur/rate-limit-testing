// cargo run -- https://www.cloudflare.com/rate-limit-test/

use std::time::*;
use std::{env, thread};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut running: bool = true;

    // get arguments
    let url = String::from(args.get(1).unwrap());
    let delay = str::parse::<u64>(args.get(2).unwrap()).unwrap();

    // let now = chrono::Local::now();
    let now = SystemTime::now();
    let mut count: u64 = 0;

    loop {
        let are_we_throttled = SystemTime::now();
        let res = send_request(url.as_str()).await;
        if are_we_throttled.elapsed().unwrap().as_millis() > 300
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
        count += 1;
        println!(
            "{}, Time elapsed: {}, Requests sent: {}",
            res,
            now.elapsed().unwrap().as_secs(),
            count
        );

        if !res.contains("200") {
            running = false;
        }
        if !running {
            println!("Requests blocked after: {}ms", count);
            break;
        }
        thread::sleep(Duration::new(delay, 0));
    }
}

async fn send_request(url: &str) -> String {
    println!("{}", url);
    reqwest::get(url).await.unwrap().status().to_string()
}
