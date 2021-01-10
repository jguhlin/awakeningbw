use crate::*;
use bevy::prelude::*;
use bevy::math::clamp;
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::na as na;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use na::Vector;
use na::Vector2;
use bevy_rapier2d::physics::{RapierPhysicsPlugin, EventQueue};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_character.system())
            .add_system_to_stage(stage::POST_UPDATE, print_events.system())

            .add_system(player_movement.system());
    }
}

pub struct Player;

fn load_character(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle =
        asset_server.load("KenneyToonCharacters/character_maleAdventurer_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(68.0, 110.0), 9, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let rigid_body1 = RigidBodyBuilder::new_dynamic().lock_rotations();
    let collider1 = ColliderBuilder::cuboid(32.0, 55.0);
    //commands.spawn((rigid_body1, collider1)); //, Player));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            // transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Player)
        .with_bundle((rigid_body1, collider1));
}

fn player_movement(
    mut app_state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &RigidBodyHandleComponent)>,
    time: Res<Time>,
    mut cam_query: Query<(&Camera2d, &mut Camera, &mut Transform)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let speed = 10000.0;

    if app_state.current() != &AppState::PlayerTurn {
        return;
    }

    let (_, mut t, mut rb) = query.iter_mut().nth(0).unwrap();
    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().2;

    let mut direction = Vector2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
        // t.translation.y += (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
        // t.translation.x -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
        // t.translation.y -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
        // t.translation.x += (speed * time.delta_seconds_f64()) as f32;
    }

    // t.translation.x = clamp(t.translation.x, -3905.0085, 3893.007);
    // t.translation.y = clamp(t.translation.y, -2120.242, 2145.111);

    let mut rb = rigid_bodies.get_mut(rb.handle()).unwrap();
    let mut curpos = rb.position().clone();
    let dir = na::Translation::from_vector(time.delta_seconds_f64() * direction * speed);

    // curpos.translation.x += dir.x as f32;
    // curpos.translation.y += dir.y as f32;

    // curpos.translation.x = clamp(curpos.translation.x, -3905.0085, 3893.007);
    // curpos.translation.y = clamp(curpos.translation.y, -2120.242, 2145.111);

    // rb.set_next_kinematic_position(curpos);
    rb.set_linvel(Vector2::new(dir.x as f32, dir.y as f32), true);

    camera_transform.translation.x = curpos.translation.x;
    camera_transform.translation.y = curpos.translation.y;
    // println!("{:#?}", t.translation);
}

fn print_events(events: Res<EventQueue>) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}