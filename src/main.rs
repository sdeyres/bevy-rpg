mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod enemy;
mod inventory;
mod map;
mod particles;
mod save;
mod state;

use std::path::MAIN_SEPARATOR;

use bevy::{prelude::*, window::WindowMode};

use crate::{
    map::generate::{poll_map_generation, prepare_tilemap_handles_resource, setup_generator}, state::GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("src{MAIN_SEPARATOR}assets"),
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
        .add_systems(Startup, prepare_tilemap_handles_resource)
        .add_systems(OnEnter(GameState::Loading), setup_generator)
        .add_systems(
            Update,
            poll_map_generation.run_if(in_state(GameState::Loading)),
        )
        .run();
}
