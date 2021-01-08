#![allow(warnings)]
#![feature(duration_zero)]

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::pass::ClearColor;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::render::RapierRenderPlugin;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::str::FromStr;

mod editor;
mod furniture;
mod player;

use editor::*;
use furniture::*;
use player::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppState {
    PlayerTurn,
    EscMenu,
    Editor,
}

pub const STAGE: &str = "app_state";

struct GreetTimer(Timer);
pub struct Camera2d;

#[derive(Default)]
pub struct StageEntities {
    entities: Vec<Entity>,
}

pub struct Player {}

#[bevy_main]
fn main() {
    let mut app = App::build();
    app.add_resource(StageEntities::default())
        .add_resource(State::new(AppState::PlayerTurn))
        .add_resource(Player {})
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(FurnitureAssets::default())
        .add_resource(WindowDescriptor {
            title: "Awakening BW".to_string(),
            width: 640.,
            height: 960.,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        // Basic Bevy Systems
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // app.add_system(bevy::input::system::exit_on_esc_system.system())
    // Universal systems
    app.add_startup_system(setup.system())
        .add_startup_system(spawn_furniture_assets.system())
        // Game resources
        // .add_resource(State::new(AppState::Overworld))
        // Generic plug-ins
        // .add_plugin(RapierPhysicsPlugin)
        // .add_plugin(PickingPlugin)
        // .add_plugin(InteractablePickingPlugin)
        // .add_plugin(TownPlugin)
        // Game-specific Plugins
        // .add_plugin(OceanPlugin)
        // .add_plugin(DinnerPlugin)
        // .add_plugin(EscMenuPlugin)
        // .add_plugin(EditorPlugin)
        // Debug systems
        .add_system(keyboard_shortcuts.system())
        .add_resource(GreetTimer(Timer::from_seconds(15.0, true)))
        .add_system(debug.system())
        .add_plugin(EditorPlugin)
        .add_plugin(PlayerPlugin)
        // .add_plugin(RapierRenderPlugin)
        // .add_plugin(DebugPickingPlugin)
        .run();
}

pub fn spawn_furniture(
    commands: &mut Commands,
    furniture_assets: Res<FurnitureAssets>,
    placed_furniture: &PlacedFurniture,
    x: f32,
    y: f32,
) {
    let asset = furniture_assets
        .assets
        .get(&placed_furniture.item)
        .unwrap()
        .clone();

    commands.spawn(SpriteBundle {
        material: asset,
        transform: {
            let mut x = Transform::from_translation(Vec3::new(x, y, 0.0));
            x.rotate(placed_furniture.rot);
            x
        },
        ..Default::default()
    });
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle {
            /*transform: Transform::from_matrix(Mat4::from_rotation_translation(
                //                Quat::from_xyzw(0.0, 0.0, 0.0, 0.0).normalize(),
                Quat::from_xyzw(-0.21, 0.0, 0.0, 0.977),
                // Vec3::new(3.72, 4.8, 13.38),
                Vec3::new(-3.45, 2.5, -2.5),
                // Vec3::new(0.0, 0.0, 0.0),
            ))
            .looking_at(Vec3::new(0.0, 0.2, 0.0), Vec3::unit_y()), */
            ..Default::default()
        })
        .with(Camera2d)
        /* .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        }) */
        .spawn(CameraUiBundle::default());

    /*    let circle = CircleShape {
        radius: 100.0,
        ..Default::default()
    };

    let mut tessellator = Tessellator::only_fill();

    commands.spawn(circle.generate_sprite(
        materials.add(ColorMaterial::color(Color::AQUAMARINE)),
        &mut meshes,
        &mut tessellator,
        &TessellationMode::Fill(&FillOptions::default()),
        Transform::default())); */

    // let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
    let texture_handle = asset_server.load("room.png");
    // let texture_handle = asset_server.load("bevy_logo_light.png");

    commands.spawn(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let texture_handle = asset_server.load("couch.png");
    // let texture_handle = asset_server.load("bevy_logo_light.png");

    commands.spawn(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let texture_handle = asset_server.load("bevy_logo_light.png");

    commands.spawn(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
}

fn debug(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Camera, &mut Transform, &Camera2d)>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for (i, (mut options, mut transform, _)) in query.iter_mut().enumerate() {
        println!("Camera {}\n {:#?}", i, transform);
    }
}

fn keyboard_shortcuts(
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut cam_query: Query<(&Camera, &mut Transform, &Camera2d)>,
    time: Res<Time>,
) {
    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().1;

    /* let speed = 50.0;

    if keyboard_input.pressed(KeyCode::W) {
        camera_transform.translation.y += (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.translation.x -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::S) {
        camera_transform.translation.y -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::D) {
        camera_transform.translation.x += (speed * time.delta_seconds_f64()) as f32;
    } */

    if keyboard_input.just_pressed(KeyCode::E) && *app_state.current() != AppState::Editor {
        app_state.set_next(AppState::Editor).unwrap();
    }

    if keyboard_input.just_pressed(KeyCode::E) && *app_state.current() == AppState::Editor {
        camera_transform.scale = Vec3::splat(1.0);
        app_state.set_next(AppState::PlayerTurn).unwrap();
    }

    /*

    if keyboard_input.just_pressed(KeyCode::H) && *app_state.current() != AppState::Town {
        app_state.set_next(AppState::Town).unwrap();
    }

    if keyboard_input.just_pressed(KeyCode::E) && *app_state.current() != AppState::Editor {
        app_state.set_next(AppState::Editor).unwrap();
    }

    if keyboard_input.just_pressed(KeyCode::O) && *app_state.current() != AppState::Overworld {
        app_state.set_next(AppState::Overworld).unwrap();
    } */
}
