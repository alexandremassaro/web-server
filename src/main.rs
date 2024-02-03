use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_address = env::var("CHAT_SERVER_ADDRESS").expect("CHAT_SERVER_ADDRESS is not set!");
    let server_port : u16 = env::var("CHAT_SERVER_PORT").expect("CHAT_SERVER_PORT is not set!").parse().expect("CHAT_SERVER_PORT must be an integer!");

    let server_add_show = match server_address.as_str() {
        "0.0.0.0" => "localhost",
        "127.0.0.1" => "localhost",
        _ => &server_address
    };

    println!("Chat server is running on http://{}:{}", server_add_show, server_port);

    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::chat)
            // .service(web::scope("/app")
            //     .route("/index.html", web::get().to(index)),)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind((server_address, server_port))?
    .run()
    .await
}