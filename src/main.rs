// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const PLAYER_MOVEMENT_SPEED: f32 = 200.;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            // Embedd assets folder into binary file.
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(
                // Comment this if you're not using pixel art.
                // This sets image filtering to nearest
                // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                // by linear filtering.
                ImagePlugin::default_nearest(),
            ),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, (spawn_2d_camera,))
        .run();
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
