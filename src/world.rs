use crate::{
    animation::AnimationTimer,
    gun::{Gun, GunTimer},
    player::Player,
    *,
};
use bevy::{math::vec3, prelude::*, time::Stopwatch};
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decoration),
        );
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
    commands.spawn((
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 17,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Gun,
        GunTimer(Stopwatch::new()),
    ));
    next_state.set(GameState::InGame);
}

fn spawn_world_decoration(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.random_range(-WORLD_W..WORLD_W);
        let y = rng.random_range(-WORLD_H..WORLD_H);
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: rng.random_range(24..=25),
                },
            ),
            Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ));
    }
}
