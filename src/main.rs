use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use z_attack_game::animation::AnimationsPlugin;
use z_attack_game::camera::CameraPlugin;
use z_attack_game::collision::CollisionPlugin;
use z_attack_game::enemy::EnemyPlugin;
use z_attack_game::gun::GunPlugin;
use z_attack_game::player::PlayerPlugin;
use z_attack_game::world::WorldPlugin;
use z_attack_game::*;

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
        //plugins
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationsPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
