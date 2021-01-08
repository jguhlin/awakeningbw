use bevy::math::{quat, Quat};
use bevy::prelude::*;
use hashbrown::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FurnitureItem {
    Couch,
    Plant,
    Piano,
}

#[derive(Debug)]
pub struct PlacedFurniture {
    pub item: FurnitureItem,
    pub x: f32,
    pub y: f32,
    pub rot: Quat,
    pub rotation: f32,
}

impl Default for PlacedFurniture {
    fn default() -> Self {
        PlacedFurniture {
            item: FurnitureItem::Couch,
            x: 0.0,
            y: 0.0,
            rot: Quat::identity(),
            rotation: 1.0,
        }
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
