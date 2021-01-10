use bevy::math::{quat, Quat};
use bevy::prelude::*;
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FurnitureItem {
    Couch,
    Plant,
    Piano,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PlacedFurniture {
    pub item: FurnitureItem,
    pub x: f32,
    pub y: f32,
    pub rot: Quat,
    pub rotation: f32,
    pub day: Day,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Day {
    All,
    Day(u8),
}

impl Day {
    pub fn next(&mut self) {
        *self = match self {
            Day::All => Day::Day(0),
            Day::Day(x) => Day::Day(*x + 1),
        }
    }

    // Should only be used during the editor!
    pub fn prev(&mut self) {
        *self = match self {
            Day::All => Day::All,
            Day::Day(0) => Day::All,
            Day::Day(x) => Day::Day(*x - 1),
        }
    }
}

impl Default for PlacedFurniture {
    fn default() -> Self {
        PlacedFurniture {
            item: FurnitureItem::Couch,
            x: 0.0,
            y: 0.0,
            rot: Quat::identity(),
            rotation: 1.0,
            day: Day::All,
        }
    }
}

impl Default for Day {
    fn default() -> Self {
        Day::Day(0)
    }
}

#[derive(Default)]
pub struct FurnitureAssets {
    pub assets: HashMap<FurnitureItem, Handle<ColorMaterial>>,
}

pub fn spawn_furniture_assets(
    mut commands: &mut Commands,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut furniture_assets: ResMut<FurnitureAssets>,
) {
    furniture_assets.assets.insert(
        FurnitureItem::Couch,
        materials.add(asset_server.load("couch.png").into()),
    );
    furniture_assets.assets.insert(
        FurnitureItem::Plant,
        materials.add(asset_server.load("plant1.png").into()),
    );
    furniture_assets.assets.insert(
        FurnitureItem::Piano,
        materials.add(asset_server.load("piano.png").into()),
    );
}
