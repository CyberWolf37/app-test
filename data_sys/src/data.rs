pub trait MongoDoc: Serialize {
    fn insert(&self,&DataCollection);
    fn delete(&self,&DataCollection);
    fn update(&self, modification: Document, &DataCollection);
}

enum DataStatus {
    Insert(Document),
    Update(Document,Document),
    Delete(Document),
}

struct DataCollection {
    handle_coll: Collection,
    pub handle_database: &'static str,
}

impl DataCollection {

    fn new(database_name: &'static str, col: Collection) -> Self {
        DataCollection {
            handle_coll: col,
            handle_database: database_name,
        }
    }

    fn get_collection(&self) -> &Collection {
        self.get_collection()
    }
}