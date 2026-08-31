use bevy::prelude::*;

#[derive(Resource, Default, PartialEq, Eq)]
pub struct CollisionMapBuilt(pub bool);
