use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;
use mongodb::results as MongoResults;
use mongodb::error as MongoError;

use crate::models;

#[derive(Debug, Clone)]
pub struct MongoClient {
    pub client: Client,
    credentials: MongoCredentials,
}

impl MongoClient {
    pub async fn new() -> Result<MongoClient, actix_web::Error> {
        let credentials = MongoCredentials::new();
        let client = create_mongo_client(&credentials).await
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError("Invalid MongoDB credentials")
            })
            .unwrap();

        let db_list = client.list_database_names(mongodb::bson::doc! {}, None).await;
        // // let transaction_options = mongodb::options::TransactionOptions::builder().build();
        // let options = mongodb::options::SessionOptions::builder().build();
        //     // .default_transaction_options(transaction_options)
        // // let session = client.start_session(options).await?;
        match db_list {
            Ok(_) => Ok(MongoClient{ client, credentials }),
            Err(_) => Err(actix_web::error::ErrorInternalServerError("Invalid MongoDB credentials")),
        }
        
    }

    async fn insert_document<T: Serialize>(
        &self, 
        collection: &str, 
        data: T, 
    ) -> MongoError::Result<MongoResults::InsertOneResult> {
        
        let coll : Collection<T> = self.client.database(self.credentials.db_name.as_str()).collection::<T>(collection);

        coll.insert_one(data, None).await
    }

    pub async fn insert_user(&self, user: &mut models::User) -> MongoError::Result<MongoResults::InsertOneResult> {
        user.created_at = Some(mongodb::bson::DateTime::now());
        self.insert_document("users", user).await
    }
}

#[derive(Debug, Clone)]
struct MongoCredentials {
    user: String,
    pwd: String,
    address: String,
    port: String,
    db_name: String,
}

impl MongoCredentials {
    fn new() -> MongoCredentials {
        let user = env::var("MONGODB_USER").expect("Database user is not defined!");
        let pwd = env::var("MONGODB_PASS").expect("Database password is not defined!");
        let address = env::var("MONGODB_ADDR").expect("Database server address is not defined!");
        let port = env::var("MONGODB_PORT").expect("Database server port is not defined!");
        let db_name = env::var("MONGODB_DB_NAME").expect("Database name is not defined!");

        MongoCredentials { user, pwd, address, port, db_name }
    }
}

fn get_mongo_uri(credentials: &MongoCredentials) -> String {
    format!("mongodb://{}:{}@{}:{}", credentials.user, credentials.pwd, credentials.address, credentials.port)
}

async fn create_mongo_client(credentials: &MongoCredentials) -> MongoError::Result<Client> {

    let mongo_uri = get_mongo_uri(credentials);
    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub db_name: String,
    pub coll_name: String,
}