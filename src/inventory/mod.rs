mod inventory;
mod systems;

use bevy::prelude::*;

pub use inventory::{Inventory, ItemKind, Pickable};

use crate::{inventory::systems::handle_pickups, state::GameState};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(Update, handle_pickups.run_if(in_state(GameState::Playing)));
    }
}
