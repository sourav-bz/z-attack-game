use std::{f32::consts::PI, time::Duration};

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    state::commands,
    time::common_conditions::on_timer,
    transform,
};
use rand::Rng;

use crate::{
    animation::AnimationTimer, player::Player, GameState, GlobalTextureAtlas, ENEMY_HEALTH,
    ENEMY_SPAWN_INTERVAL, ENEMY_SPEED, MAX_NUM_ENEMIES, SPAWN_RATE_PER_SECOND, SPRITE_SCALE_FACTOR,
    WORLD_H, WORLD_W,
};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * ENEMY_SPEED;
        transform.translation.z = 10.0;
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies: u32 = enemy_query.iter().len() as u32;
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 12,
                },
            ),
            Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Enemy::default(),
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        ));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..PI * 2.0);
    let dist = rng.random_range(100.0..2000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x + 10.0;
    let random_y = pos.y + offset_y + 10.0;

    (random_x, random_y)
}

fn despawn_dead_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter_mut() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}
