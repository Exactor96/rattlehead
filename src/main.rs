use std::env;
use std::process::exit;

mod bot;
mod web;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "bot" {
        bot::start_bot().await;
    } else if &args[1] == "web" {
        web::start_web().await;
    }
    {
        exit(1);
    }
}
