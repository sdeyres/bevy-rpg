use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldSeed(pub u64);

pub fn init_single_player_seed(mut commands: Commands) {
    let seed = rand::random();
    info!("Single player world seed: {}", seed);
    commands.insert_resource(WorldSeed(seed));
}
