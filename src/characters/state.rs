use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CharacterState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
}

impl CharacterState {
    pub fn is_grounded(&self) -> bool {
        matches!(self, Self::Idle | Self::Walking | Self::Running)
    }
}
