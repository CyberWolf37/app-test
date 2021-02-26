use mongodb::{
    Client, 
    options::{
        ClientOptions,
        DeleteOptions,
        FindOptions,
        UpdateModifications,
        Hint,
    },
    error::Error as DBError,
    Collection,
    Database,
    bson::{
        Document,
        ser as bsonser,
        doc,
    },
};

use futures::prelude::*;
use tokio::prelude::*;
use tokio::runtime::{ Runtime, TaskExecutor };
use serde::{ Serialize, Deserialize};
use std::sync::Arc;
use log::{info,warn};
use env_logger::Logger;
use data::{ DataCollection, DataStatus};

async fn connection<'a>(url_root: &str, database: &str, collections: &'a [&str]) -> Option<Vec<DataCollection>> {

    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse(url_root).await;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options.unwrap());

    if let Ok(client) = client {

        let mut stack_collection: Vec<DataCollection> = Vec::new();

        collections.iter().for_each(|item| {
            let collection = client.database(database).collection(item);
            let handle = DataCollection::new(database, collection);
            info!("Connecté à la collection {}", item);
            stack_collection.push(handle);

        });

        return Some(stack_collection);
        
    }
    else {
        warn!("Problème de connection à l'url {}",url_root);
        None
    }
}

struct DataManager {
    list_collections: Vec<DataCollection>,
    stack_tasks: Arc<Vec<Arc<dyn fn()>>>,
    url_root: String,
}

impl DataManager {
    fn new(url_root: &str) -> Self {
        DataManager{
            list_collections: Vec::new(),
            url_root: String::from(url_root),
            stack_tasks: Arc::new(Vec::new()),
        }
    }

    fn connect<'a>(mut self, database: &str, collections: &'a [&str]) {
        info!("Connect to database {} in progress ", database);

        let mut rt = tokio::runtime::Runtime::new().unwrap();

        let stack_connection = rt.block_on(connection(self.url_root.as_str(),database,collections));

        if let Some(collect) = stack_connection {
            self.list_collections = collect;
        }
        else {
            warn!("Some trouble appear")
        }
    }

    fn launch(&self) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let executor = rt.executor();
        let task = self.stack_tasks.clone().first();

        executor.spawn(future::lazy(|| {
            Ok(())
        }));
    }

    fn insert<'a>(&self, dataState: DataStatus,collection: &'a DataCollection) {

        let func = move || {
            match dataState {
                DataStatus::Insert(doc) => {
                    collection.insert_one(doc,None);
                },
                DataStatus::Delete(query) => {
                    collection.delete_one(query,None);
                },
                DataStatus::Update(query,doc) => {
                    collection.update_one(query,doc,None);
                },
            }
        };

        self.stack_tasks.clone().push(func);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataManager;
        use crate::data::DataCollection;
        use crate::prelude::MongoDoc;
        use futures::prelude::*;
        use tokio::prelude::*;
        use serde::{Serialize, Deserialize};
        use mongodb::bson::doc;

        #[derive(Serialize, Deserialize)]
        struct Profil {
            name: String,
            say: String,
        }

        env_logger::init();

        let array_collection = ["profil"];

        let dataManager = DataManager::new("mongodb://localhost:27017");
        dataManager.connect("test-app", &array_collection);

        let mut profil = Profil{name: "Cyber".to_string(),say: "Hello".to_string()};
    }
}
