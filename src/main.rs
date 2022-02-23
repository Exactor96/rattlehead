use actix_web::{App, HttpRequest, HttpServer, middleware, web};
use env_logger;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

    println!("Starting on port: {}", port);

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "<h1>Hello world!</h1>" }))
            .service(web::resource("/").to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
