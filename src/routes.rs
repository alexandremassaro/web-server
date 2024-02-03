use actix_web::{web, get, post, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Dona Carlota client service.")
}

#[post("/chat")]
async fn chat(message_received: web::Query<MessageReceived>) -> impl Responder {
    println!("{:?}", message_received);
    HttpResponse::Ok().body(format!("
        Chat room : {}\n
        - {}: {}
        ", 
        message_received.conversation_id,
        message_received.sender_id,
        message_received.message,
    ))
}

#[derive(Debug, Deserialize)]
struct MessageReceived {
    conversation_id: String,
    sender_id: String,
    message: String,
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }