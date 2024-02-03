use mongodb::{Client, options::ClientOptions, error};
use std::env;

pub struct MongoClient {client: Client}

impl MongoClient {
    pub async fn new() -> MongoClient {
        let client = create_mongo_client().await.expect("Error when trying to connect to database");
        MongoClient{ client }
    }
}

struct MongoCredentials {
    user: String,
    pwd: String,
    address: String,
    port: String,
}

impl MongoCredentials {
    fn new() -> MongoCredentials {
        dotenv::dotenv().ok(); 

        let user = env::var("MONGODB_USER").expect("Database user is not defined!");
        let pwd = env::var("MONGODB_PASS").expect("Database password is not defined!");
        let address = env::var("MONGODB_ADDR").expect("Database server address is not defined!");
        let port = env::var("MONGODB_PORT").expect("Database server port is not defined!");

        MongoCredentials { user, pwd, address, port }
    }
}

fn get_mongo_uri() -> String {
    let credentials = MongoCredentials::new();
    format!("mongodb://{}:{}@{}:{}", credentials.user, credentials.pwd, credentials.address, credentials.port)
}

async fn create_mongo_client() -> error::Result<Client> {

    let mongo_uri = get_mongo_uri();
    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}

