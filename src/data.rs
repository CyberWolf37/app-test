use mongodb::bson::{ Document };
use mongodb::sync::{
    Collection
};
use serde::Serialize;
use std::sync::Arc;
use log::{ info, warn};
use mongodb::options::UpdateModifications;

