use mongodb::{Client, options::ClientOptions, error::Error as DBError, Collection, Database};
use log::{info,warn};
use env_logger::Logger;
use futures::prelude::*;
use tokio::prelude::*;
use mongodb::bson::doc;
use mongodb::bson::Document;
use serde::{ Serialize, Deserialize};

pub trait Data {
    type Item;
    fn insert(&self) -> Document;
    fn delete(&self) -> Document;
    fn update(&self) -> Document;
}

struct DataSys {
    url_root: String,
    handle_coll: Option<Collection>,
    db: Option<String>,
    collection: Option<String>,
}

impl DataSys {

    fn new(url: &str) -> Self {
        info!("[DATA SYS] Set url {}",url);
        DataSys {
            url_root: url.to_string(),
            handle_coll: None,
            db: None,
            collection: None,
        }
    }

    fn in_db(mut self, db: &str) -> Self {
        self.db = Some(db.to_string());
        self
    }

    fn in_collection(mut self, collection: &str) -> Self {
        self.collection = Some(collection.to_string());
        self
    }

    async fn connection(&mut self) {

        info!("[DATA SYS] Try to connect to {}",self.url_root);
        // Parse a connection string into an options struct.
        let client_options = ClientOptions::parse(self.url_root.as_str()).await;

        // Get a handle to the deployment.
        let client = Client::with_options(client_options.unwrap());

        if let Ok(client) = client {

            if self.db.is_some() & self.collection.is_some() {
                let db_name = self.db.clone();
                let collection_name = self.collection.clone();

                let handle_coll = client.database(db_name.unwrap().as_ref())
                    .collection(collection_name.unwrap().as_ref());

                self.handle_coll = Some(handle_coll);
                
            }
            
        }
    }

    async fn insert<'a>(&mut self, data: impl Serialize) {
        
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataSys;
        use futures::prelude::*;
        use tokio::prelude::*;

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let mut data_sys = DataSys::new("mongodb://localhost:27017")
            .in_db("Test-app")
            .in_collection("logging");

        rt.block_on(data_sys.connection());
        println!("Hello");
        
    }
}
