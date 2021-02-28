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
use data::{ DataCollection, DataStatus, MongoDoc};

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
    stack_tasks: Vec<Arc<dyn FnOnce() + 'static + Send + Sync>>,
    url_root: String,
}

impl DataManager {
    fn new(url_root: &str) -> Self {
        DataManager{
            list_collections: Vec::new(),
            url_root: String::from(url_root),
            stack_tasks: Vec::new(),
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
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let task = self.stack_tasks.clone().first();
    }

    fn insert<'a>(&self, dataState: DataStatus,data: impl MongoDoc, collection: &str) {

        let coll = self.find_coll(collection);

        if let Some(collection) = coll {
            let func: Arc<dyn FnOnce() + 'static + Send + Sync>  = 
            match dataState {
                DataStatus::Insert => {
                    data.insert(collection.clone())
                },
                DataStatus::Delete => {
                    data.delete(collection)
                },
                DataStatus::Update(doc) => {
                    data.update(doc, collection)
                },
            };

            self.stack_tasks.clone().push(func);
        }
        else {
            warn!("Sorry doesn't have this collection : {}",collection );
        }

        
    }

    fn find_coll(&self,name: &str) -> Option<Arc<DataCollection>> {
        let result = self.list_collections.iter().find(|x| x.get_name_coll() == name );
        
        if let Some(result) = result {
            Some(result.clone())
        }
        else {
            None
        }
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::DataManager;
        use crate::data::{DataCollection ,DataStatus ,MongoDoc};
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

        #[derive(Serialize, Deserialize)]
        struct Profil {
            name: String,
            say: String,
        }

        impl MongoDoc for Profil {
            fn insert(&self, dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +Send +Sync> {
                let bson = bsonser::to_document(&self);
                let collection = dataColl.clone();

                let func = move || {
                    if let Ok(doc) = bson {
                        collection.clone().get_collection().insert_one(doc,None);
                    }
                    else{
                        warn!("Some probleme occurs on insert manager")
                    }
                };

                Arc::new(func)
            }
            fn delete(&self, dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +Send +Sync> {
                let document = bsonser::to_document(&self);
                let collection = dataColl.clone();

                let func = move || {
                    if let Ok(document) = document {
                        let keyname= document.get("name");

                        if let Some(keyname) = keyname {
                            let query = doc! { "name": keyname };
                            let result = collection.clone().get_collection().delete_one(query,None);

                            if let Ok(result) = result {
                                info!("Object deleted 👍");
                            }
                            
                        }
                    }
                };

                Arc::new(func)
            }
            fn update(&self, modification: Document, dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +Send +Sync> {
                let document = bsonser::to_document(&self);
        
                let func = move || {
                    if let Ok(document) = document {

                        let query = doc! { "name": document.get("name").unwrap()};

                        let func = || {
                            
                            let result = dataColl.clone().get_collection().update_one(query,modification,None);

                            if let Ok(_) = result {
                                info!("Object Update 👍");
                            }
                        };
                    }
                };

                Arc::new(func)
            }
        }

        env_logger::init();

        let array_collection = ["profil"];

        let mut dataManager = DataManager::new("mongodb://localhost:27017");
        dataManager.connect("test-app", &array_collection);

        let mut profil = Profil{name: "Cyber".to_string(),say: "Hello".to_string()};

        dataManager.insert(DataStatus::Insert, profil, "profil");
    }
}
