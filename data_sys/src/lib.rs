use mongodb::{Client, options::ClientOptions, error::Error as DBError, Collection, Database};
use log::{info,warn};
use env_logger::Logger;
use futures::prelude::*;
use tokio::prelude::*;
use mongodb::options::DeleteOptions;
use mongodb::options::Hint;
use mongodb::bson::doc;
use mongodb::bson::ser as bsonser;
use mongodb::bson::Document;
use serde::{ Serialize, Deserialize};

async fn connection<'a>(url_root: &str, database: &str, collections: &'a [&str]) -> Option<Vec<Collection>> {

    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse(url_root).await;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options.unwrap());

    if let Ok(client) = client {

        let stack_collection: Vec<Collection> = Vec::new();

        collections.iter().for_each(|item| {
            let handle = client.database(database).collection(item);
            
            match handle {
                Ok(handle) => {
                    info!("Connecté à la collection {}", item);
                    stack_collection.push(handle);
                },
                Err(_) => {
                    warn!("Problème de connection sur la collection {}",item);
                }
            }
        })

        if self.db.is_some() & self.collection.is_some() {
            let db_name = self.db.clone();
            let collection_name = self.collection.clone();

            let handle_coll = client.database(db_name.unwrap().as_ref())
                .collection(collection_name.unwrap().as_ref());

            self.handle_coll = Some(handle_coll);
            
        }
        
    }
    else {
        warn!("Problème de connection à l'url {}",url_root);
        None
    }
}

struct DataManager {
    list_collections: Vec<Collection>,
    list_db: Vec<Database>,
    url_root: String,
}

impl DataManager {
    fn new(url_root: &str) -> Self {
        DataManager{
            list_collections: Vec::new(),
            list_db: Vec::new(),
            url_root: String::from(url_root),
        }
    }

    fn connect<'a>(mut self, database: &str, collections: &'a [&str]) {
        info!("Connect to database {} in progress ", database);

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let stack_connection = rt.block_on(connection(&self.url_root,database,collections));

    }
}

struct DataSys {
    url_root: String,
    handle_coll: Option<Collection>,
    db: Option<String>,
    collection: Option<String>,
}

impl DataSys {

    fn new(url: &str) -> Self {
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

        info!("Try to connect to {}",self.url_root);
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

    async fn insert<'a>(&self, data: &'a impl Serialize) {
        let bson = bsonser::to_document(data);

        if let Ok(doc) = bson {
            self.handle_coll.clone().unwrap().insert_one(doc,None).await;
            info!("Object inserted 👍")
        }
        else{
            warn!("Can not insert document in database {}, on Collection {}",self.db.as_ref().unwrap(),self.collection.as_ref().unwrap())
        }
    }

    async fn delete<'a>(&mut self, data: &'a impl Serialize,finder: &str) {
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataSys;
        use futures::prelude::*;
        use tokio::prelude::*;
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct Profil{
            name: String,
            say: String,
        }

        let profil = Profil{name: "Cyber".to_string(),say: "Hello".to_string()};
        env_logger::init();

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let mut data_sys = DataSys::new("mongodb://localhost:27017")
            .in_db("test-app")
            .in_collection("profil");

        rt.block_on(data_sys.connection());
        rt.block_on(data_sys.insert(&profil));       
    }
}
