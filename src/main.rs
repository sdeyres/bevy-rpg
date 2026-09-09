mod audio;
mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod enemy;
mod inventory;
mod map;
mod module_bindings;
mod network;
mod particles;
mod save;
mod state;

use std::path::MAIN_SEPARATOR;

use bevy::{prelude::*, window::WindowMode};

use crate::{
    map::{poll_map_generation, prepare_tilemap_handles_resource, setup_generator},
    state::GameState,
};

fn main() {
    let assets_path = get_assets_path();

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: assets_path,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy RPG".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(state::StatePlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(characters::CharactersPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(particles::ParticlesPlugin)
        .add_plugins(save::SavePlugin)
        .add_plugins(audio::AudioManagerPlugin)
        .add_plugins(network::NetworkPlugin)
        .add_systems(Startup, prepare_tilemap_handles_resource)
        .add_systems(
            OnEnter(GameState::Loading),
            setup_generator.run_if(not(state::in_multiplayer)),
        )
        .add_systems(
            Update,
            poll_map_generation
                .run_if(in_state(GameState::Loading))
                .run_if(not(state::in_multiplayer)),
        )
        .run();
}

fn get_assets_path() -> String {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_assets = exe_dir.join("assets");
            if exe_assets.exists() {
                return exe_assets.to_string_lossy().to_string();
            }
        }
    }
    format!("src{}assets", MAIN_SEPARATOR)
}
