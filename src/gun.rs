use std::f32::consts::PI;
use std::time::Instant;

use crate::player::Player;
use crate::resources::{CursorPosition, GlobalTextureAtlas};
use crate::*;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct SpawnInstant(Instant);

#[derive(Component)]
pub struct BulletDirection(Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_gun_input,
                update_bullets,
                update_gun_transform,
                despawn_old_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, entity) in bullet_query.iter_mut() {
        if instant.0.elapsed().as_secs_f32() > BULLET_LIFE_TIME_IN_SECS {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let mut rng = rand::rng();
    let bullet_direction = gun_transform.local_x();
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        for _ in 0..NUM_OF_BULLET_PER_SHOT {
            let dir = vec3(
                bullet_direction.x + rng.random_range(-1.0..1.0),
                bullet_direction.y + rng.random_range(-1.0..1.0),
                bullet_direction.z,
            );
            commands.spawn((
                Sprite::from_atlas_image(
                    handle.image.clone().unwrap(),
                    TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: 16,
                    },
                ),
                Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 11.0)),
                Bullet,
                BulletDirection(dir),
                SpawnInstant(Instant::now()),
            ));
        }
    }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut transform, direction) in bullet_query.iter_mut() {
        transform.translation += direction.0.normalize() * Vec3::splat(BULLET_SPEED);
        transform.translation.z = 10.0;
    }
}

fn update_gun_transform(
    cursor_pos: ResMut<CursorPosition>,
    player_query: Query<&mut Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 15.0,
    );
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 10.0;
}
