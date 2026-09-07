use bevy::prelude::*;

use crate::combat::events::EntityDeath;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, commands: &mut Commands, entity: Entity, amount: f32) {
        self.current = (self.current - amount).max(0.);

        if !self.is_alive() {
            commands.trigger(EntityDeath { entity });
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.
    }

    pub fn ratio(&self) -> f32 {
        self.current / self.max
    }
}
