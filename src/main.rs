use std::env;
use std::process::exit;
use tokio::runtime::Runtime;

mod bot;
mod web;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "bot" {
        bot::start_bot().await;
    } else if &args[1] == "web" {
        web::start_web().await;
    } else if &args[1] == "all"{
        let mut rt = Runtime::new().unwrap();
        rt.spawn(async move {
            tokio::spawn(async {bot::start_bot().await});
            tokio::spawn(async {web::start_web().await});
        });
            // loop {}


    }
    {
        exit(1);
    }
}
