use bevy::prelude::*;
use crate::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_character.system())
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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 20.0), 9, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle, 
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..Default::default()
    }).with(Player);
}

fn player_movement(mut app_state: ResMut<State<AppState>>, 
    input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    mut cam_query: Query<(&Camera2d, &mut Camera, &mut Transform)>,
)
{
    let speed = 500.0;

    if app_state.current() != &AppState::PlayerTurn {
        return;
    }

    let mut t = query.iter_mut().nth(0).unwrap().1;
    let mut camera_transform = cam_query.iter_mut().nth(0).unwrap().2;

    if keyboard_input.pressed(KeyCode::W) {
        t.translation.y += (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::A) {
        t.translation.x -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::S) {
        t.translation.y -= (speed * time.delta_seconds_f64()) as f32;
    }

    if keyboard_input.pressed(KeyCode::D) {
        t.translation.x += (speed * time.delta_seconds_f64()) as f32;
    }

    camera_transform.translation = t.translation;

}