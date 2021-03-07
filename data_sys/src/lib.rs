extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, Lit, Meta, MetaNameValue};
use derive_map::{DataStatus, DataCollection, Task};


#[proc_macro_derive(MongoDoc)]
pub fn mongoDoc(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    

    let name = &input.ident;

    let expanded = quote! {
        impl derive_map::MongoDoc for #name {
            fn insert(&self, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self);
                Arc::new(Task::new(DataStatus::Insert, dataColl, document.unwrap(), None, None))
            }
            fn delete(&self, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self).unwrap();

                let keyname= document.get("name");
                if let Some(keyname) = keyname {
                    let query = doc! { "name": keyname };   
                    Arc::new(Task::new(DataStatus::Delete, dataColl, bsonser::to_document(&self).unwrap(), Some(query), None))                 
                }
                else {
                    panic!("Failed to get the query");
                }
            }
            fn update(&self, modification: Document, dataColl: Arc<DataCollection>) -> Arc<Task> {
                let document = bsonser::to_document(&self).unwrap();

                let keyname = document.get("name");
                if let Some(keyname) = keyname {
                    let query = doc! { "name": keyname };   
                    Arc::new(Task::new(DataStatus::Update, dataColl, bsonser::to_document(&self).unwrap(), Some(query), Some(modification)))                 
                }
                else {
                    panic!("Failed to get the query");
                }
            }
        }
    };

    TokenStream::from(expanded)
}