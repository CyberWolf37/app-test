use mongodb::bson::{ Document };
use mongodb::sync::{
    Collection
};
use serde::Serialize;
use std::sync::Arc;

pub trait MongoDoc: Serialize {
    fn insert(&self,dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +'static +Send +Sync>;
    fn delete(&self,dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +'static +Send +Sync>;
    fn update(&self, modification: Document,dataColl: Arc<DataCollection>) -> Arc<dyn FnOnce() +'static +Send +Sync>;
}

pub enum DataStatus {
    Insert,
    Delete,
    Update(Document),
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
        self.get_collection()
    }

    pub fn get_name_coll(&self) -> &str {
        self.get_collection().name()
    }

    pub fn query() {

    }
}