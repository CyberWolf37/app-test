use mongodb::bson::{ Document };
use mongodb::sync::{
    Collection
};
use serde::Serialize;
use std::sync::Arc;
use log::{ info, warn};
use mongodb::options::UpdateModifications;

pub trait MongoDoc: Serialize {
    fn insert(&self,dataColl: Arc<DataCollection>) -> Arc<Task>;
    fn delete(&self,dataColl: Arc<DataCollection>) -> Arc<Task>;
    fn update(&self, modification: Document,dataColl: Arc<DataCollection>) -> Arc<Task>;
}

#[derive(Clone)]
pub struct Task {
    status: DataStatus,
    collection: Arc<DataCollection>,
    document: Document,
    query: Option<Document>,
    modification: Option<Document>,
}

impl Task {
    pub fn new(status: DataStatus, collection: Arc<DataCollection>, document: Document, query: Option<Document>, modification: Option<Document>) -> Self {
        Task {
            status: status,
            collection: collection,
            document: document,
            query: query,
            modification: modification,
        }
    }

    pub fn consume(self) {
        match &self.status {
            DataStatus::Insert => {
                let handle = self.collection.get_collection().insert_one(self.document ,None);
                
                match handle {
                    Ok(msg) => info!("Object inserted 👍 : {:?}", msg),
                    Err(e) => warn!("Object has not inserted 😧 : {}",e)
                }
            },
            DataStatus::Delete => {
                if let Some(query) = self.query {
                    let handle = self.collection.clone().get_collection().delete_one(query,None);
                    
                    match handle {
                        Ok(msg) => info!("Object deleted 👍 : {:?}", msg),
                        Err(e) => warn!("Object has not deleted 😧 : {}",e)
                    }
                }
                else {
                    warn!("Object doesn't have query 🔥")
                }
            }
            DataStatus::Update => {
                if let Some(query) = self.query {

                    if let Some(docu) = self.modification {
                       let handle = self.collection.clone().get_collection().update_one(query,UpdateModifications::Document(docu),None);

                       match handle {
                        Ok(msg) => info!("Object updated 👍 : {:?}", msg),
                        Err(e) => warn!("Object has not updated 😧 : {}",e)
                    }
                    }
                    else {
                        warn!("Object doesn't have query 🔥")
                    }
                    
                }
                else {
                    warn!("Object doesn't have query 🔥")
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum DataStatus {
    Insert,
    Delete,
    Update,
}

pub struct DataCollection {
    handle_coll: Collection,
    pub handle_database: &'static str,
}

impl DataCollection {

    pub fn new(database_name: &'static str, col: Collection) -> Self {
        DataCollection {
            handle_coll: col,
            handle_database: database_name,
        }
    }

    pub fn get_collection(&self) -> &Collection {
        &self.handle_coll
    }

    pub fn get_name_coll(&self) -> &str {
        self.get_collection().name()
    }

    pub fn query() {

    }
}