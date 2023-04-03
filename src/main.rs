mod parse;
mod msg;

use dotenv::dotenv;
use chrono::{Local, TimeZone, Datelike, Duration};
use clap::Parser;
use parse::Args;
use msg::msg_handler;


#[tokio::main]
async fn main() {

    // Parse command-line arguments
    let args = Args::parse();
    // Parse .env file
    dotenv().ok();

    // set reminder time. 
    let now = chrono::Local::now();

    let mut next = Local
        .with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            args.hour,
            args.minute,
            0)
        .unwrap();

    // set the value for when the reminder repeats
    let inc = Duration::hours(args.interval);

    // make sure that the coming reminder is in the future
    while next < now {
        next = next + inc;
    }

    println!("Seconds until next reminder: {}", next.timestamp() - now.timestamp());

    loop {
        while Local::now() < next {
            // sleep for a bunch of seconds
            std::thread::sleep(
                std::time::Duration::new(
                    ( next.timestamp() - Local::now().timestamp() ) as u64 , 0
                )
            );
        }
        // set new reminder time before awaiting msg
        next += inc;
        msg_handler().await;
    }

    
}


