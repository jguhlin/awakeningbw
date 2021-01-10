use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::math::clamp;
use bevy::math::quat;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::PerspectiveProjection;
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

use crate::*;
use furniture::*;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Setup
            .add_startup_system(setup_world.system()) 
            .on_state_enter(
                STAGE,
                AppState::PlayerTurn,
                spawn_the_days_furniture.system(),
            )
            // Update
            .on_state_update(STAGE, AppState::PlayerTurn, change_day.system())
            // .on_state_update(STAGE, AppState::PlayerTurn, click.system())
            // .on_state_update(STAGE, AppState::PlayerTurn, keyboard_shortcuts.system())
            // .on_state_update(STAGE, AppState::PlayerTurn, hover.system())
            // .on_state_update(STAGE, AppState::PlayerTurn, hover_update.system())
            // .on_state_update(STAGE, AppState::PlayerTurn, world_pos.system())
            // Exit
            .on_state_exit(STAGE, AppState::PlayerTurn, cleanup_entities.system());
    }
}

fn cleanup_entities(commands: &mut Commands, mut entities: ResMut<StageEntities>) {
    for i in entities.entities.drain(..) {
        commands.despawn_recursive(i);
    }
}

pub fn setup_world(commands: &mut Commands) {
    // 8000 x 4516
    let width: f32 = 8000.0;
    let height: f32 = 4386.0;

    // bottom wall

    let rigid_body1 = RigidBodyBuilder::new_static().translation(0.0, -(height/2.0+12.5));
    let collider1 = ColliderBuilder::cuboid(width/2.0, 25.0);
    commands.spawn((rigid_body1, collider1));

    // top wall
    let rigid_body1 = RigidBodyBuilder::new_static().translation(0.0, (height/2.0+12.5));
    let collider1 = ColliderBuilder::cuboid(width/2.0, 25.0);
    commands.spawn((rigid_body1, collider1));

    // right wall

    let rigid_body1 = RigidBodyBuilder::new_static().translation((width/2.0-25.5), 0.0);
    let collider1 = ColliderBuilder::cuboid(25.0, height/2.0);
    commands.spawn((rigid_body1, collider1));

    // furthest left wall

    let rigid_body1 = RigidBodyBuilder::new_static().translation(-(width/2.0-25.5), 0.0);
    let collider1 = ColliderBuilder::cuboid(25.0, height/2.0);
    commands.spawn((rigid_body1, collider1));



}

pub fn spawn_the_days_furniture(
    mut commands: &mut Commands,
    furniture_assets: Res<FurnitureAssets>,
    mut camera_query: Query<(Entity, &mut Camera, &mut Transform, &Camera2d)>,
    mut stage_entities: ResMut<StageEntities>,
    mut furniture_entities: ResMut<FurnitureEntities>,
    day: Res<Day>,
) {
    // TODO: Move elsewhere
    /* for (e, x, mut i, _) in camera_query.iter_mut() {
        i.scale = Vec3::new(1.00, 1.00, 1.00);
    } */ 

    let furniture = FURNITURE.get().unwrap().read().unwrap();

    let mut remaining_entities = Vec::new();

    for (iday, i) in furniture_entities.entities.drain(..) {
        println!("Checking {:#?}", iday);
        if iday == Day::All || iday == *day {
            remaining_entities.push((iday, i));
        } else {
            println!("Despawning...");
            commands.despawn_recursive(i);
        }
    }

    furniture_entities.entities = remaining_entities;

    for i in furniture.iter() {
        if i.day == *day || i.day == Day::All {
            let e = spawn_furniture(&mut commands, &furniture_assets, &i);
            stage_entities.entities.push(e);
            furniture_entities.entities.push((i.day, e));
        }
    }
}

pub fn change_day(
    mut commands: &mut Commands,
    furniture_assets: Res<FurnitureAssets>,
    mut camera_query: Query<(Entity, &mut Camera, &mut Transform, &Camera2d)>,
    mut stage_entities: ResMut<StageEntities>,
    mut furniture_entities: ResMut<FurnitureEntities>,
    day: ChangedRes<Day>,
    dayr: Res<Day>,
) {
    println!("ChangedRes called...");
    spawn_the_days_furniture(
        &mut commands,
        furniture_assets,
        camera_query,
        stage_entities,
        furniture_entities,
        dayr,
    );
}
