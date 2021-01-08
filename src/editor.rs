use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::math::clamp;
use bevy::math::quat;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::PerspectiveProjection;

use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

use crate::*;
use furniture::*;

#[derive(Default)]
pub struct EditorData {
    world_pos: Vec2
}

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
            .add_resource(EditorData::default())
            .on_state_enter(STAGE, AppState::Editor, editor_create.system())
            // Update
            .on_state_update(STAGE, AppState::Editor, click.system())
            .on_state_update(STAGE, AppState::Editor, keyboard_shortcuts.system())
            .on_state_update(STAGE, AppState::Editor, hover.system())
            .on_state_update(STAGE, AppState::Editor, hover_update.system())
            .on_state_update(STAGE, AppState::Editor, world_pos.system())
            // Exit
            .on_state_exit(STAGE, AppState::Editor, cleanup_entities.system());
    }
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn cleanup_entities(commands: &mut Commands, mut entities: ResMut<StageEntities>) {
    for i in entities.entities.drain(..) {
        commands.despawn_recursive(i);
    }
}

fn editor_create(
    commands: &mut Commands,
    furniture_assets: Res<FurnitureAssets>,
    mut camera_query: Query<(Entity, &mut Camera, &mut Transform, &Camera2d)>,
    mut stage_entities: ResMut<StageEntities>,
) {
    for (e, x, mut i, _) in camera_query.iter_mut() {
        i.scale = Vec3::new(1.00, 1.00, 1.00);
    }

    let placed_furniture = PlacedFurniture::default();

    commands.spawn(SpriteBundle {
        material: furniture_assets
        .assets
        .get(&placed_furniture.item)
        .unwrap()
        .clone(),
        transform: {
            let mut x = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
            x.rotate(placed_furniture.rot);
            x
        },
        ..Default::default()
    }).with(Hovering).with(placed_furniture);

    stage_entities.entities.push(commands.current_entity().unwrap());
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn world_pos(
    mut state: Local<MouseState>,
    windows: Res<Windows>,
    mut cam_query: Query<(&Camera2d, &mut Camera, &mut Transform)>,
    mut editor_data: ResMut<EditorData>,
) {
    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().2;

    let window = windows.get_primary().unwrap();
    let cursor_position = match window.cursor_position() {
        Some(x) => x,
        None => return,
    };

    let size = Vec2::new(window.width() as f32, window.height() as f32);
    let p = cursor_position - size / 2.0;
    let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
    editor_data.world_pos.x = world_pos.x;
    editor_data.world_pos.y = world_pos.y;
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
    furniture_assets: Res<FurnitureAssets>,
    placed_furniture: Query<(&Hovering, &PlacedFurniture)>,
    mut cam_query: Query<(&Camera2d, &mut Camera, &mut Transform)>,
) {
    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().2;
    let mut placed_furniture = placed_furniture.iter().nth(0).unwrap().1;

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let cursor_position = window.cursor_position().unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = cursor_position - size / 2.0;
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        println!("Spawning... {} {} {:#?}", world_pos.x, world_pos.y, placed_furniture);

        spawn_furniture(
            &mut commands,
            furniture_assets,
            &placed_furniture,
            world_pos.x,
            world_pos.y,
        );
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        camera_transform.scale.x = clamp(camera_transform.scale.x - event.y, 1.0, 20.0);
        camera_transform.scale.y = clamp(camera_transform.scale.y - event.y, 1.0, 20.0);
    }
}

struct Hovering;

fn hover(mut state: Local<State>,
//    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(&Hovering, Entity, &mut Transform)>,
    mut editor_data: ResMut<EditorData>,
) {

    let mut hovering_item = query.iter_mut().nth(0).unwrap().2;

    //for i in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        hovering_item.translation.x = editor_data.world_pos.x;
        hovering_item.translation.y = editor_data.world_pos.y;
    //}
}

fn hover_update(furniture_assets: Res<FurnitureAssets>, 
    mut query: Query<(&Hovering, Entity, &mut Handle<ColorMaterial>, &PlacedFurniture, &mut Transform), Mutated<PlacedFurniture>>) {

    for (_, _, mut x, placed_furniture, mut transform) in query.iter_mut() {
        *x = furniture_assets
            .assets
            .get(&placed_furniture.item)
            .unwrap()
            .clone();

        transform.rotation = placed_furniture.rot;
    }
}

fn keyboard_shortcuts(
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut placed: Query<(&Hovering, &mut PlacedFurniture)>,
    mut cam_query: Query<(&Camera, &mut Transform, &Camera2d)>,
    time: Res<Time>,
) {

    let mut placed = placed.iter_mut().nth(0).unwrap().1;

    if keyboard_input.just_pressed(KeyCode::R) {
        placed.rotation += 1.0;

        if placed.rotation == 4.0 {
            placed.rotation = 0.0;
        }

        println!("Rotation factor: {:#?}", placed.rotation);

        placed.rot = Quat::from_rotation_z((std::f32::consts::PI / 2.0) * placed.rotation);
    }

    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().1;

    if keyboard_input.just_pressed(KeyCode::Key1) {
        placed.item = FurnitureItem::Couch;
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        placed.item = FurnitureItem::Plant;
    }

    if keyboard_input.just_pressed(KeyCode::Key9) {
        placed.item = FurnitureItem::Piano;
    }

    let speed = 500.0;

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
