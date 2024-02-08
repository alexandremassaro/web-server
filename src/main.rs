use actix_web::{middleware::Logger, App, HttpServer, web};
use actix_web_lab::web::spa;
use dotenv::dotenv;
use std::env;
use db::MongoClient;
use common::{db, models};

// mod models;
mod routes;
// mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)] 
    {
        println!("Debug mode. 🪲💣🪦⚰️");
        dotenv().ok();
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }

    let db_client = get_mongodb_client().await.expect("Not possible to create a client to MongoDB!");

    let server_address = env::var("CHAT_SERVER_ADDRESS").expect("CHAT_SERVER_ADDRESS is not set!");
    let server_port : u16 = env::var("CHAT_SERVER_PORT").expect("CHAT_SERVER_PORT is not set!").parse().expect("CHAT_SERVER_PORT must be an integer!");

    let server_add_show = match server_address.as_str() {
        "0.0.0.0" => "localhost",
        "127.0.0.1" => "localhost",
        _ => &server_address
    };

    println!("Chat server is running on http://{}:{}", server_add_show, server_port);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(db_client.clone()))
            // .service(routes::index)
            .service(routes::chat)
            .service(web::scope("/user")
                .service(routes::user_test)
                .service(routes::new_user)
                .service(routes::get_all_users))
            .service(
                spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish()
            )
    })
    .bind((server_address, server_port))?
    .run()
    .await
}

async fn get_mongodb_client() -> Option<MongoClient> {
    match db::MongoClient::new().await {
        Ok(client) => {
            println!("MongoDB client created successfully!");
            Some(client)
        },
        Err(e) => {
            println!("Error creating MongoDB client.");
            println!("{}", e);
            None
        },
    }
}