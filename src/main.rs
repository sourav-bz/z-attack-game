use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const WW: f32 = 1200.0;
const WH: f32 = 900.0;
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

//sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 8;
const SPRITE_SHEET_H: u32 = 8;

//player
const PLAYER_SPEED: f32 = 2.0;

//resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource)]
struct CursorPosition(Option<Vec2>);

//components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

//state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        //custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPosition(None))
        //systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(Update, (handle_player_input, update_gun_transform, update_cursor_position).run_if(in_state(GameState::InGame)))
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>, 
    mut image_handle: ResMut<GlobalSpriteSheetHandle>, 
    asset_server: Res<AssetServer>, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>
){
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_W, TILE_H), SPRITE_SHEET_W, SPRITE_SHEET_H, None, None);
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));
    next_state.set(GameState::GameInit);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera{..Default::default()}));
}

fn init_world(
    mut commands: Commands, 
    texture_atlas: Res<GlobalTextureAtlasHandle>, 
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>
){
    commands.spawn((
        Sprite::from_atlas_image(
            image_handle.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Player
    ));
    commands.spawn((
        Sprite::from_atlas_image(
            image_handle.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 17,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Gun
    ));
    next_state.set(GameState::InGame);
}

fn handle_player_input(mut query: Query<&mut Transform, With<Player>>, keyboard_input: Res<ButtonInput<KeyCode>>){

    if query.is_empty(){
        return;
    }

    let mut transform = query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let d_key = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    
    let mut delta = Vec2::ZERO;

    if w_key{
        delta.y += 1.0;
    }
    if s_key{
        delta.y -= 1.0;
    }
    if a_key{
        delta.x -= 1.0;
    }
    if d_key{
        delta.x += 1.0;
    }

    delta = delta.normalize();

    if delta.is_finite() && (w_key || s_key || a_key || d_key){
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
    }
    
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
){
    if window_query.is_empty() || camera_query.is_empty(){
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    cursor_pos.0 = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
}

fn update_gun_transform(
    cursor_pos: ResMut<CursorPosition>,
    player_query: Query<&mut Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>
){
    if player_query.is_empty() || gun_query.is_empty(){
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0{
        Some(pos) => pos,
        None => player_pos
    };
    let mut gun_transform = gun_query.single_mut();

    // let to_player = (cursor_pos - gun_transform.translation.truncate()).normalize();
    // let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

    // gun_transform.rotation = rotate_to_player;

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0, 
        player_pos.y + offset * angle.sin() - 15.0
    );
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
}