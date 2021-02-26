use mongodb::bson::{ Document };
use mongodb::{Collection};
use serde::Serialize;

pub trait MongoDoc: Serialize {
    fn insert(&self,dataColl: &DataCollection) -> dyn Fn() +Send +Sync;
    fn delete(&self,dataColl: &DataCollection) -> dyn Fn() +Send +Sync;
    fn update(&self, modification: Document,dataColl: &DataCollection) -> dyn Fn() +Send +Sync;
}

pub enum DataStatus {
    Insert(Document),
    Delete(Document),
    Update(Document,Document),
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

    pub fn query() {

    }
}