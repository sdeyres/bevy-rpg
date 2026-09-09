mod animation;
mod collider;
mod config;
mod facing;
mod input;
mod physics;
mod rendering;
mod spawn;
mod state;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{collision::CollisionMapBuilt, state::GameState};

pub use animation::{AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME};
pub use collider::Collider;
pub use config::{CharacterEntry, CharactersList};
pub use facing::Facing;
pub use input::Player;
pub use physics::{Velocity, calculate_velocity};
pub use spawn::{CharactersListResource, CurrentCharacterIndex, PlayerSpawned};
pub use state::CharacterState;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .init_resource::<PlayerSpawned>()
            .add_systems(Startup, spawn::load_character_assets)
            .add_systems(
                Update,
                spawn::spawn_player_at_valid_position
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(PlayerSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    input::handle_player_input,
                    spawn::switch_character,
                    input::update_jump_state,
                    animation::on_state_change_update_animation,
                    collider::validate_movement,
                    collider::resolve_entity_collisions,
                    physics::apply_velocity,
                    rendering::update_character_depth,
                    animation::animations_playback,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
