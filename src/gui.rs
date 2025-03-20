use bevy::{
    color::palettes::css::GOLD, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*
};

use crate::GameState;

pub struct GUIPlugin;

#[derive(Component)]
pub struct FpsText;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text,
            );
    }
}

fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     Text::new("hello\nbevy!"),
    //     TextFont {
    //         // This font is loaded and will be used instead of the default font.
    //         font: asset_server.load("monogram.ttf"),
    //         font_size: 50.0,
    //         ..default()
    //     },
    // ));
    commands
        .spawn((
            // Create a Text with multiple child spans.
            Text::new("FPS: "),
            TextFont {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("monogram.ttf"),
                font_size: 42.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            if cfg!(feature = "default_font") {
                (
                    TextFont {
                        font_size: 33.0,
                        // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                        ..default()
                    },
                    TextColor(GOLD.into()),
                )
            } else {
                (
                    // "default_font" feature is unavailable, load a font to use instead.
                    TextFont {
                        font: asset_server.load("monogram.ttf"),
                        font_size: 33.0,
                        ..Default::default()
                    },
                    TextColor(GOLD.into()),
                )
            },
            FpsText,
        ));
}

fn update_debug_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    if query.is_empty() {
        return;
    }

    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
    
}
