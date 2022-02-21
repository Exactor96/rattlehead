use warp::Filter;

pub async fn start_web() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
