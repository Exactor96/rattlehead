use actix_web::{App, get, HttpRequest, HttpServer, middleware, post, Responder, web};
use env_logger;
use serde::Deserialize;
use teloxide::prelude2::*;

#[derive(Deserialize)]
struct Message {
    text: String,
    title: Option<String>,
    source: Option<String>,
}

#[get("/ping")]
async fn ping_handler(_req: HttpRequest) -> impl Responder {
    "pong"
}

#[post("/send_message/{chat_id}")]
async fn send_message_handler(_bot: web::Data<AutoSend<Bot>>, path: web::Path<i64>, message_data: web::Json<Message>) -> impl Responder {
    let mut message = format!("{}", message_data.text);
    if message_data.title.is_some() {
        message = format!("{}\n\n{}", message_data.title.as_ref().unwrap(), message);
    }
    if message_data.source.is_some(){
        message = format!("{}\n\nSource: {}", message, message_data.source.as_ref().unwrap());
    }
    let chat_id = path.into_inner();

    let bot = _bot.get_ref();

    let result = bot.send_message(chat_id, message).await;
    match result {
        Ok(_)=> "Sent successfully".to_string(),
        Err(error) => format!("Error: {:?}", error)
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

    println!("Starting on port: {}", port);

    let bot = Bot::from_env().auto_send();

    HttpServer::new(move || {
        App::new()
            // enable logger
            .app_data(web::Data::new(bot.clone()))
            .wrap(middleware::Logger::default())
            .service(send_message_handler)
            .service(ping_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
