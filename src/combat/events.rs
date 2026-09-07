use bevy::prelude::*;

use crate::combat::PowerType;

#[derive(Event)]
pub struct ProjectileHit {
    pub target: Entity,
    pub damage: f32,
    pub power_type: PowerType,
}

#[derive(Event)]
pub struct EntityDeath {
    pub entity: Entity,
}
