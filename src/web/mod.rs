use warp::Filter;
use pretty_env_logger;

pub async fn start_web() {
    pretty_env_logger::init();
    println!("WEB STARTED");
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
