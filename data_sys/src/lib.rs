mod data;

use mongodb::{
    sync::{ 
        Client,
        Collection,
        Database,
    },
    options::{
        ClientOptions,
        DeleteOptions,
        FindOptions,
        UpdateModifications,
        Hint,
    },
    error::Error as DBError,
    
    bson::{
        Document,
        ser as bsonser,
        doc,
    },
};

use futures::prelude::*;
use tokio::prelude::*;
use tokio::runtime::{ Runtime };
use serde::{ Serialize, Deserialize};
use std::sync::Arc;
use log::{info,warn};
use env_logger::Logger;
use data::{ DataCollection, DataStatus, MongoDoc, Task};
use std::thread;
use std::sync::Mutex;
use std::marker::Copy;


fn connection<'a>(url_root: &str, database: &'static str, collections: &'a [&str]) -> Option<Vec<Arc<DataCollection>>> {

    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse(url_root);

    // Get a handle to the deployment.
    let client = Client::with_options(client_options.unwrap());

    if let Ok(client) = client {

        let mut stack_collection: Vec<Arc<DataCollection>> = Vec::new();

        collections.iter().for_each(|item| {
            let collection = client.database(database).collection(item);
            let handle = DataCollection::new(database, collection);
            info!("Connecté à la collection {}", item);
            stack_collection.push(Arc::new(handle));

        });

        return Some(stack_collection);
        
    }
    else {
        warn!("Problème de connection à l'url {}",url_root);
        None
    }
}

struct DataManager {
    list_collections: Vec<Arc<DataCollection>>,
    stack_tasks: Arc<Mutex<Vec<Arc<Task>>>>,
    url_root: String,
}

impl DataManager {
    fn new(url_root: &str) -> Self {
        DataManager{
            list_collections: Vec::new(),
            url_root: String::from(url_root),
            stack_tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn connect<'a>(&mut self, database: &'static str, collections: &'a [&str]) {
        info!("Connect to database {} in progress ", database);

        let stack_connection = connection(self.url_root.as_str(),database,collections);

        if let Some(collect) = stack_connection {
            self.list_collections = collect;
        }
        else {
            warn!("Some trouble appear")
        }
    }

    fn launch(&self) {
        let stack = self.stack_tasks.clone();
        thread::spawn(move || {
            loop {
                if let Ok(mut stack_task) = stack.lock() {

                    if let Some(task) = stack_task.pop() {
                       task.consume();
                    }  
                }
            }
        });
    }

    fn insert<'a>(&mut self, dataState: DataStatus,data: impl MongoDoc, collection: &str) {

        let coll = self.find_coll(collection);

        if let Some(collection) = coll {
            let task: Arc<Task>  = 
                match dataState {
                    DataStatus::Insert => {
                        info!("Is an insert value");
                        data.insert(collection.clone())
                    },
                    DataStatus::Delete => {
                        info!("Is an delete value");
                        data.delete(collection)
                    },
                    DataStatus::Update(doc) => {
                        info!("Is an update value");
                        data.update(doc, collection)
                    },
                };

            if let Ok(mut stack_func) = self.stack_tasks.lock() {
                stack_func.push(task);
            }
        }
        else {
            warn!("Sorry doesn't have this collection : {}",collection );
        }
 
    }

    fn find_coll(&self,name: &str) -> Option<Arc<DataCollection>> {
        let result = self.list_collections.iter().find(|x| x.clone().get_name_coll() == name );
        
        if let Some(result) = result {
            Some(result.clone())
        }
        else {
            warn!("Sorry don't have the collection {}",name);
            None
        }
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataManager;
        use crate::data::{DataCollection ,DataStatus ,MongoDoc ,Task};
        use mongodb::bson::{
            ser as bsonser,
            Document,
        };
        use futures::prelude::*;
        use tokio::prelude::*;
        use serde::{Serialize, Deserialize};
        use mongodb::bson::doc;
        use log::{ info, warn};
        use std::sync::Arc;
        use std::thread;
        use std::sync::Mutex;

        #[derive(Serialize, Deserialize)]
        struct Profil {
            name: String,
            say: String,
        }

        impl MongoDoc for Profil {
            fn insert(&self, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self);
                let collection = dataColl.clone();
                Arc::new(Task::new(DataStatus::Insert, dataColl, document.unwrap(), None))
            }
            fn delete(&self, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self);
                let collection = dataColl.clone();

                let keyname= document.unwrap().get("name");
                if let Some(keyname) = keyname {
                    let query = doc! { "name": keyname };   
                    Arc::new(Task::new(DataStatus::Delete, dataColl, document.unwrap(), Some(query)))                 
                }
                else {
                    panic!("Failed to get the query");
                }
            }
            fn update(&self, modification: Document, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self);
                let collection = dataColl.clone();

                let keyname= document.unwrap().get("name");
                if let Some(keyname) = keyname {
                    let query = doc! { "name": keyname };   
                    Arc::new(Task::new(DataStatus::Update(modification), dataColl, document.unwrap(), Some(query)))                 
                }
                else {
                    panic!("Failed to get the query");
                }
            }
        }

        env_logger::init();

        let array_collection = ["profil"];

        let mut dataManager = DataManager::new("mongodb://localhost:27017");
        dataManager.connect("test-app", &array_collection);

        let profil = Profil{name: "Cyber".to_string(),say: "Hello".to_string()};

        let document = bsonser::to_document(&profil);

        if let Ok(document) = document {
            info!("Document as parsed = {}",document);
        }
        else {
            warn!("Failed to parse document");
        }

        dataManager.insert(DataStatus::Insert, profil, "profil");

        thread::sleep(std::time::Duration::from_secs(2));

        dataManager.launch();
    }
}
