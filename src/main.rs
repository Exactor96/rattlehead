use actix_multipart::Multipart;
use actix_web::{App, get, http::header, HttpRequest, HttpServer, middleware, post, Responder, web};
use env_logger;
use serde::Deserialize;
use teloxide::{prelude2::*, prelude::StreamExt, types::InputFile};
use uuid::Uuid;

#[derive(Deserialize)]
struct Message {
    text: String,
    title: Option<String>,
    source: Option<String>,
}


const MAX_FILE_SIZE: usize = 104857600; // 100mb

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

#[post("/send_attachment/{chat_id}")]
async fn send_attachment_handler(_bot: web::Data<AutoSend<Bot>>, path: web::Path<i64>, mut payload: Multipart, req: HttpRequest) -> web::Json<Vec<String>> {
    let chat_id = path.into_inner();
    let content_length = req.headers().get(header::CONTENT_LENGTH).unwrap().to_str().unwrap().parse::<usize>().unwrap();

    if content_length <= 0 || content_length > MAX_FILE_SIZE{
        return web::Json(vec![format!("Content Length must be greater than 0 and less than {}. Current content length: {}", MAX_FILE_SIZE, content_length)]);
    }

    let bot = _bot.as_ref();
    let mut file_names: Vec<String> = Vec::new();

    while let Some(field) = payload.next().await {
        // A multipart/form-data stream has to contain `content_disposition`
        let mut field = field.unwrap();
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let mut body = web::BytesMut::new();

        // Field in turn is stream of *Bytes* object
        while let Some( chunk) = field.next().await {
            let _chunk = chunk.unwrap();
            body.extend_from_slice(&_chunk);
        }

        let file= InputFile::memory(body).file_name(filename.clone());
        let result = bot.send_document(chat_id, file).await;
        if Some(result).is_some() {
            file_names.push(filename);
        }
    }

    // format!("{} files was sent. Files: [{}]", files_count, file_names.join(", ")).to_string()
    return web::Json(file_names);
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
            .service(send_attachment_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
