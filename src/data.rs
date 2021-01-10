use bevy::prelude::*;
use hashbrown::HashMap;
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use yaml_rust::{yaml::Yaml, YamlLoader, YamlEmitter};
use std::fs;
use std::fs::File;
use std::io::prelude::*;


pub static FURNITURE: OnceCell<Arc<RwLock<Vec<PlacedFurniture>>>> = OnceCell::new();

use crate::furniture::*;
use crate::*;

pub fn init_data() {
    if FURNITURE.get().is_some() {
        return;
    }

    let x: Vec<PlacedFurniture> = match fs::read_to_string("data/furniture.yaml") {
        Ok(x) => serde_yaml::from_str(&x).expect("Unablet to parse YAML"),
        Err(_) => Vec::new()
    };
//    let x = fs::read_to_string("data/furniture.yaml").expect("Unable to read YAML");
//    let x: Vec<PlacedFurniture> = serde_yaml::from_str(&x);

    FURNITURE.set(Arc::new(RwLock::new(x)));

}

pub fn record_furniture(placed: PlacedFurniture) {
    FURNITURE.get().unwrap().write().unwrap().push(placed);
}

pub fn write_yaml() {
    let x = FURNITURE.get().unwrap().read().unwrap().clone();
    let x = serde_yaml::to_string(&x).unwrap();
    let mut file = File::create("data/furniture.yaml").expect("Unable to write file");
    file.write_all(x.as_bytes()).expect("Unable to write data file");
}