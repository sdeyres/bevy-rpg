mod events;
pub mod health;
pub mod healthbar;
mod observers;
mod player_combat;
mod power_type;
pub mod systems;

use bevy::prelude::*;

pub use health::Health;
pub use healthbar::HealthBarOwner;
pub use player_combat::PlayerCombat;
pub use power_type::{PowerType, PowerVisuals};
pub use systems::{
    Projectile, ProjectileEffect, ProjectileOwner, debug_switch_power, handle_power_input,
    spawn_projectile,
};

use crate::state::GameState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observers::on_projectile_hit)
            .add_observer(observers::on_entity_death)
            .add_systems(
                Update,
                (
                    handle_power_input,
                    debug_switch_power,
                    systems::move_projectiles,
                    systems::check_projectile_hits,
                    healthbar::spawn_healthbars,
                    healthbar::update_healthbars,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
