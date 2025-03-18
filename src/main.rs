use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

const WW: f32 = 1200.0;
const WH: f32 = 700.0;
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_W: u32 = 16;
const TILE_H: u32 = 16;
const SPRITE_SHEET_W: u32 = 8;
const SPRITE_SHEET_H: u32 = 8;

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
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (setup_camera, spawn_player))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera{..Default::default()}));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>){
    let texture = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_W, TILE_H), SPRITE_SHEET_W, SPRITE_SHEET_H, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    ));
}