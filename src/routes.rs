use actix_web::{web::{self, Json}, get, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::{db, models};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Dona Carlota client service.")
}

#[post("/new")]
async fn new_user(db_client: web::Data<db::MongoClient>, user_data: web::Json<models::User>) -> impl Responder {
    if user_data.0.username.trim().is_empty() { 
        return HttpResponse::BadRequest().body(format!("username cannot be an empty string \"\""))
    }
    
    let mut user_data = models::User::from(user_data.0);
    
    println!("{:?}", user_data);

    match db_client.insert_user(&mut user_data).await {
        Ok(result) => {
            HttpResponse::Ok().body(format!("User inserted with id: {}", result.inserted_id))
        },
        Err(e) => {
            HttpResponse::BadRequest().body(format!("Error handling request: {}", e.to_string()))
        },
    }
}

#[get("all")]
async fn get_all_users(db_client: web::Data<db::MongoClient>) -> impl Responder {
    let opts = mongodb::options::FindOptions::builder()
        .sort(mongodb::bson::doc! {"username": 1})
        .build();

    let mut cursor = db_client.client.database("carlotachatdb").collection::<models::User>("users").find(None, opts).await.unwrap();

    let mut users: Vec<models::User> = vec![];

    while cursor.advance().await.unwrap() {
        let current_user = cursor.deserialize_current().unwrap();
        println!("{:?}", current_user);
        users.push(current_user);
    }
    
    let json = serde_json::to_string(&users).unwrap();
    HttpResponse::Ok().body(format!("{}", json))
}

#[post("/test")]
async fn user_test(db_client: web::Data<db::MongoClient>, req: Json<models::User>) -> impl Responder {
    format!("{:?} - {:?}", req, db_client)
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