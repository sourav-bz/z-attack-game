use bevy::prelude::*;

use crate::{
    enemy::{self, Enemy},
    gun::Gun,
    player::{Player, PlayerState},
    CursorPosition, GameState,
};

pub struct AnimationsPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                flip_player_sprite_x,
                flip_gun_sprite_y,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut Sprite, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, player_state, timer) = player_query.single_mut();
    if let Some(atlas) = &mut sprite.texture_atlas {
        if timer.just_finished() {
            let base_spirit_index = match player_state {
                PlayerState::Idle => 0,
                PlayerState::Moving => 4,
            };
            atlas.index = base_spirit_index + (atlas.index + 1) % 4;
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();

    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_enemy_sprite_x(
    mut player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, enemy_tranform) in enemy_query.iter_mut() {
        if enemy_tranform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.y < transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}
