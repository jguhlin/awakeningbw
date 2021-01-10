use crate::*;
use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::na as na;
use na::Vector2;
use once_cell::sync::OnceCell;

pub struct NpcPlugin;
impl Plugin for NpcPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app //.add_startup_system(load_assets.system())
            .add_startup_system(npc_spawner.system())
            //.add_system(npc_movement.system())
            ;
    }
}

pub struct Npc;

pub static NPCSPRITE: OnceCell<Handle<TextureAtlas>> = OnceCell::new();

fn npc_spawner(
    mut commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle =
        asset_server.load("KenneyToonCharacters/character_maleAdventurer_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(68.0, 110.0), 9, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    match NPCSPRITE.set(texture_atlas_handle.clone()) {
        Err(x) => println!("NPCSPRITE already set..."),
        _ => ()
    };

    let rigid_body1 = RigidBodyBuilder::new_dynamic().lock_rotations();
    let collider1 = ColliderBuilder::cuboid(32.0, 55.0);

    // spawn_npc(&mut commands, 100.0, 100.0);
    // spawn_npc(&mut commands, 150.0, 250.0);
}

pub fn spawn_npc(commands: &mut Commands, x: f32, y: f32) {
    let rigid_body1 = RigidBodyBuilder::new_dynamic().lock_rotations().translation(x, y);
    let collider1 = ColliderBuilder::cuboid(32.0, 55.0);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: NPCSPRITE.get().unwrap().clone(),
            transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
            ..Default::default()
        })
        .with_bundle((Npc, rigid_body1, collider1));
}

fn npc_movement(
    mut app_state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Npc, &mut Transform, &RigidBodyHandleComponent)>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let speed = 10000.0;

    let mut rng = thread_rng();

    if app_state.current() != &AppState::PlayerTurn {
        return;
    }

    // let mut t = query.iter_mut().nth(0).unwrap().1;
    for (_, mut t, mut rb) in query.iter_mut() {
        let mut rb = rigid_bodies.get_mut(rb.handle()).unwrap();

        let mut dir = Vector2::new(0.0, 0.0);
        dir.x = (rng.gen_range(-1.0..=1.0) * speed * time.delta_seconds_f64()) as f32;
        dir.y = (rng.gen_range(-1.0..=1.0) * speed * time.delta_seconds_f64()) as f32;

        rb.set_linvel(dir, true);
    }
}
