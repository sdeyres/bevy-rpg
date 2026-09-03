mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod inventory;
mod map;
mod particles;
mod state;

use std::path::MAIN_SEPARATOR;

use bevy::{prelude::*, window::WindowMode};
use bevy_procedural_tilemaps::{proc_gen::grid::Cartesian3D, simple_plugin::ProcGenSimplePlugin};

use crate::map::generate::setup_generator;

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
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_plugins(state::StatePlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(characters::CharactersPlugin)
        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(particles::ParticlesPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_systems(Startup, setup_generator)
        .run();
}
