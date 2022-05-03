use std::env;

use mongodb::{
    Client,
    options::{ClientOptions, ResolverConfig},
};

#[derive(Clone)]
pub struct Database {
    pub client: Client,
    pub db: mongodb::Database,
}

impl Database {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // Load the MongoDB connection string from an environment variable:
        let client_uri =
            env::var("DB_URL").expect("You must set the DB_URL environment var!");

        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await.unwrap();
        let client = Client::with_options(options).unwrap();

        let db_name =
            env::var("DB_NAME").expect("You must set the DB_NAME environment var!");
        let db = client.database(&db_name);

        Ok(Self {
            client,
            db,
        })
    }
}