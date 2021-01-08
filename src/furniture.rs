use bevy::math::{Quat, quat};

pub enum FurnitureItem {
    Couch,
    Plant,
    Piano,
}

impl FurnitureItem {
    pub fn get_filename(&self) -> &'static str {
        match &self {
            FurnitureItem::Couch => "couch.png",
            FurnitureItem::Plant => "plant.png",
            FurnitureItem::Piano => "piano.png",
        }
    }
}

pub struct PlacedFurniture {
    pub item: FurnitureItem,
    pub x: f32,
    pub y: f32,
    pub rot: Quat,
}

impl Default for PlacedFurniture {
    fn default() -> Self {
        PlacedFurniture {
            item: FurnitureItem::Couch,
            x: 0.0,
            y: 0.0,
            rot: Quat::identity(),
        }
    }
}