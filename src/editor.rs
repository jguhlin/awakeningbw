use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::math::quat;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::PerspectiveProjection;
use bevy::math::clamp;

use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

use crate::*;
use furniture::*;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Setup
            /*.on_state_enter(
                STAGE,
                AppState::Editor,
                .system(),
            ) */
            .add_resource(PlacedFurniture::default())
            .on_state_enter(STAGE, AppState::Editor, editor_create.system())
            // Update
            .on_state_update(STAGE, AppState::Editor, click.system())
            // Exit
            .on_state_exit(STAGE, AppState::Editor, cleanup_entities.system());
    }
}

fn cleanup_entities(commands: &mut Commands, mut entities: ResMut<StageEntities>) {
    for i in entities.entities.drain(..) {
        commands.despawn_recursive(i);
    }
}

fn editor_create(
    commands: &mut Commands,
    mut entities: ResMut<StageEntities>,
    mut camera_query: Query<(Entity, &mut Camera, &mut Transform, &Camera2d)>,
) {
    for (e, x, mut i, _) in camera_query.iter_mut() {
        i.scale = Vec3::new(1.00, 1.00, 1.00);
    }
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn click(
    mut commands: &mut Commands,
    mut state: Local<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut cam_query: Query<(&Camera2d, &mut Camera, &mut Transform)>,
) {

    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().2;

/*    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    { */
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let cursor_position = window.cursor_position().unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = cursor_position - size / 2.0;
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        spawn_couch(&mut commands, &mut materials, &mut meshes, &mut asset_server, world_pos.x, world_pos.y);
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        camera_transform.scale.x = clamp(camera_transform.scale.x - event.y, 1.0, 20.0);
        camera_transform.scale.y = clamp(camera_transform.scale.y - event.y, 1.0, 20.0);
    }
}

fn keyboard_shortcuts(
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut placed: ResMut<PlacedFurniture>,
) {

    if keyboard_input.pressed(KeyCode::Key1) {
        placed.item = FurnitureItem::Couch;
    }

    if keyboard_input.pressed(KeyCode::Key2) {
        placed.item = FurnitureItem::Plant;
    }

    if keyboard_input.pressed(KeyCode::Key9) {
        placed.item = FurnitureItem::Piano;
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