mod components;
mod material;
mod systems;

use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub use material::*;
pub use systems::*;

use crate::state::GameState;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing ParticlesPlugin...");
        app.add_plugins(Material2dPlugin::<ParticleMaterial>::default())
            .add_systems(
                Update,
                (update_emitters, update_particles, cleanup_finished_emitters)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
        info!("ParticlesPlugin initialized!")
    }
}
