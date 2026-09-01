use bevy::prelude::*;

use crate::particles::components::ParticleEmitter;

pub fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut ParticleEmitter, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
}
