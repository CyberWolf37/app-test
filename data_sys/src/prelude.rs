extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};
use crate::DataStatus;


#[proc_macro_derive(MongoDoc, attributes(findKey,mongoManager))]
pub fn mongoDoc(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl crate::MongoDoc for #name {
            fn insert(&self) {
                let bson = bsonser::to_document(&self);

                if let Ok(doc) = bson {
                    #mongoManager.insert(DataStatus::Insert(doc));
                }
                else{
                    warn!("Some probleme occurs on insert manager")
                }
            }
            fn delete(&self) {
                let document = bsonser::to_document(data);
        
                if let Ok(document) = document {
                    let keyname= document.get(#findKey);

                    if let Some(keyname) = keyname {
                        let doc = doc! { #findKey: keyname };
                        let result = #mongoManager.insert(DataStatus::Delete(doc));

                        if let Ok(result) = result {
                            info!("Object deleted 👍");
                        }
                        
                    }
                }
            }
            fn update(&self, modification: Document) {
                let document = bsonser::to_document(data);
        
                if let Ok(document) = document {

                    let query = doc! { findKey: document.get(findKey).unwrap()};
                    
                    let result = mongoManager.insert(DataStatus::Update(query, modification,None).await;

                    if let Ok(_) = result {
                        info!("Object Update 👍");
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}