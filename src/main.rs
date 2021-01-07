#![allow(warnings)]
#![feature(duration_zero)]

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::render::RapierRenderPlugin;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rand_distr;
use std::str::FromStr;
use bevy::render::camera::Camera;
use bevy::render::pass::ClearColor;
use bevy_prototype_lyon::prelude::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppState {
    DisplayMenu,
    PlayerTurn,
    EnemyTurn,
    Win,
    Lose,
}

pub const STAGE: &str = "app_state";

struct GreetTimer(Timer);

#[derive(Default)]
pub struct StageEntities {
    entities: Vec<Entity>,
}

pub struct Player {
    total_hp: u32,
    current_hp: u32,
}

pub struct Mob {
    total_hp: u32,
    current_hp: u32,
}

#[bevy_main]
fn main() {
    let mut app = App::build();
    app.add_resource(StageEntities::default())
        .add_resource(State::new(AppState::PlayerTurn))
        .add_resource(Player { total_hp: 32, current_hp: 32 })
        .add_resource(Mob { total_hp: 32, current_hp: 32 })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "Awakening: BW".to_string(),
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

        app.add_system(bevy::input::system::exit_on_esc_system.system())
        // Universal systems
        .add_startup_system(setup.system())
        // Game resources
        // .add_resource(State::new(AppState::Overworld))
        // Generic plug-ins
        .add_plugin(RapierPhysicsPlugin)
        // .add_plugin(PickingPlugin)
        // .add_plugin(InteractablePickingPlugin)
        // .add_plugin(TownPlugin)
        // Game-specific Plugins
        // .add_plugin(OceanPlugin)
        // .add_plugin(DinnerPlugin)
        // .add_plugin(EscMenuPlugin)
        // .add_plugin(EditorPlugin)
        // Debug systems
        // .add_plugin(FlyCameraPlugin)
        .add_system(keyboard_shortcuts.system())
        .add_resource(GreetTimer(Timer::from_seconds(15.0, true)))
        .add_system(debug.system())
        // .add_plugin(RapierRenderPlugin)
        // .add_plugin(DebugPickingPlugin)
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());

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
        /* .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        }) */
        .spawn(CameraUiBundle::default());

}

fn debug(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for (mut options, mut transform) in query.iter_mut() {
        // transform.scale *= 0.9;
        println!("{:#?}", transform);
        println!("Done...");
    }
}

fn keyboard_shortcuts(
    input: Res<Input<KeyCode>>,
    // mut query: Query<&mut FlyCamera>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    /*
    for mut options in query.iter_mut() {
        
        if input.just_pressed(KeyCode::T) {
            println!("Toggled FlyCamera enabled!");
            options.enabled = !options.enabled;
        }

        if keyboard_input.just_pressed(KeyCode::H) && *app_state.current() != AppState::Town {
            app_state.set_next(AppState::Town).unwrap();
        }

        if keyboard_input.just_pressed(KeyCode::E) && *app_state.current() != AppState::Editor {
            app_state.set_next(AppState::Editor).unwrap();
        }

        if keyboard_input.just_pressed(KeyCode::O) && *app_state.current() != AppState::Overworld {
            app_state.set_next(AppState::Overworld).unwrap();
        }
    } */
}