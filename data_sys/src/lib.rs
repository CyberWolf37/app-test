use mongodb::{Client, options::ClientOptions, error::Error as DBError};
use log::{info,warn};
use env_logger::Logger;
use futures::prelude::*;
use tokio::prelude::*;
use mongodb::bson::doc;
use mongodb::bson::Document;

pub trait Data {
    type Item;
    fn insert(&self) -> Document;
    fn delete(&self) -> Document;
    fn update(&self) -> Document;
}

struct DataSys {
    url_root: String,
    client: Option<Client>,
    db: Option<String>,
    collection: Option<String>,
}

impl DataSys {

    fn new(url: &str) -> Self {
        info!("[DATA SYS] Set url {}",url);
        DataSys {
            url_root: url.to_string(),
            client: None,
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

        self.client = Some(client.unwrap());
    }

    async fn insert<'a>(&mut self, data: impl Data) {
        let doc = doc! { "title": "1984", "author": "George Orwell" };
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataSys;
        use futures::prelude::*;
        use tokio::prelude::*;
        use 

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let mut data_sys = DataSys::new("mongodb://localhost:27017");

        rt.block_on(data_sys.connection());
        println!("Hello");
        
    }
}
