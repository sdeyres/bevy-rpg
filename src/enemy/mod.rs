mod ai;
mod combat;
mod components;
mod spawn;

use bevy::prelude::*;

pub use components::{AIBehavior, Enemy, EnemyCombat};
pub use spawn::spawn_enemy;

use crate::{collision::CollisionMapBuilt, enemy::spawn::EnemiesSpawned, state::GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemiesSpawned>()
            .add_systems(
                Update,
                spawn::spawn_test_enemies
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(EnemiesSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (ai::enemy_follow_player, combat::enemy_attack)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
