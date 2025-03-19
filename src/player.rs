use crate::*;
use bevy::{math::vec3, prelude::*};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_input(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    let mut transform = query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;

    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }

    delta = delta.normalize();

    if delta.is_finite() && (w_key || s_key || a_key || d_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        transform.translation.z = 10.0;
    }
}
