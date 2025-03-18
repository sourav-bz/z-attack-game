use std::f32::consts::PI;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::Rng;
use z_attack_game::resources::{GlobalSpriteSheetHandle, GlobalTextureAtlasHandle};
use z_attack_game::*;



//components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

#[derive(Component)]
struct GunTimer(Stopwatch);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct BulletDirection(Vec3);

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
        //external plugins
        .add_plugins(PanCamPlugin::default())
        //custom resources
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        
        //systems
        .add_systems(OnEnter(GameState::GameInit), (
                setup_camera, 
                init_world,
                spawn_world_decoration,
        ))
        .add_systems(Update, (
                handle_player_input, 
                update_gun_transform, 
                handle_gun_input,
                update_bullets,
                camera_follow_player,
            ).run_if(in_state(GameState::InGame)))
        .run();
}



fn setup_camera(mut commands: Commands) {
    commands.spawn(
        (
            Camera2d, 
            Camera{..Default::default()}
        )
    ).insert(PanCam::default());
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
        Gun,
        GunTimer(Stopwatch::new())
    ));
    next_state.set(GameState::InGame);
}

fn spawn_world_decoration(
   mut commands: Commands, 
    texture_atlas: Res<GlobalTextureAtlasHandle>, 
    image_handle: Res<GlobalSpriteSheetHandle>
){
    let mut rng = rand::rng();
    for _ in 0..NUM_WORLD_DECORATIONS{
        let x = rng.random_range(-WORLD_W..WORLD_W);
        let y = rng.random_range(-WORLD_H..WORLD_H);
        commands.spawn((
            Sprite::from_atlas_image(
                image_handle.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: rng.random_range(24..=25),
                },
            ),
            Transform::from_translation(vec3(x, y, 0.0)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ));
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
){
    if camera_query.is_empty() || player_query.is_empty(){
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 1.0), 0.1);

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
        transform.translation.z = 10.0;
    }
    
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>, 
    image_handle: Res<GlobalSpriteSheetHandle>,
    mouse_button_input: Res<ButtonInput<MouseButton>>
){
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let bullet_direction = gun_transform.local_x();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        commands.spawn((
            Sprite::from_atlas_image(
                image_handle.0.clone().unwrap(),
                TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 16,
                },
            ),
            Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 11.0)),
            Bullet,
            BulletDirection(*bullet_direction),
        ));
    }
}



fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>,
){
    if bullet_query.is_empty(){
        return;
    }

    for (mut transform, direction) in bullet_query.iter_mut(){
        transform.translation += direction.0.normalize() * Vec3::splat(BULLET_SPEED);
        transform.translation.z = 10.0;
    }
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

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0, 
        player_pos.y + offset * angle.sin() - 15.0
    );
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 11.0;
}

