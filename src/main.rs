// cargo run -- https://www.cloudflare.com/rate-limit-test/
use std::{env, thread};
use std::time::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut running: bool = true;

    // get arguments
    let url = String::from(args.get(1).unwrap());

    // let now = chrono::Local::now();
    let now = SystemTime::now();
    let mut count = 0;



    loop {

        let are_we_throttled = SystemTime::now();
        let res = send_request(url.as_str()).await;
        if are_we_throttled.elapsed().unwrap().as_millis() > 200 {
            println!("Slow...");
        }
        if are_we_throttled.elapsed().unwrap().as_millis() > 300 {
            println!("Throttled...");
        }
        count += 1;
        println!("{}, Time elapsed: {}, Requests sent: {}", res, now.elapsed().unwrap().as_secs(), count);

        if !res.contains("200") {
            running = false;
        }
        if !running {
            println!("Requests blocked after: {}", count);
            break
        }
    }


}


async fn send_request(url: &str) -> String {
    reqwest::get(url).await.unwrap().status().to_string()
}
