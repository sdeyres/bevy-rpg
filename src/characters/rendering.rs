use bevy::prelude::*;

use crate::{
    characters::input::Player,
    config::{
        map::{GRID_Y, TILE_SIZE},
        player::PLAYER_SCALE,
    },
};

const NODE_SIZE_Z: f32 = 1.;
const PLAYER_BASE_Z: f32 = 4.;
const PLAYER_Z_OFFSET: f32 = 0.5;

pub fn update_player_depth(
    mut player_query: Query<&mut Transform, (With<Player>, Changed<Transform>)>,
) {
    let map_height = TILE_SIZE * GRID_Y as f32;
    let map_y0 = -TILE_SIZE * GRID_Y as f32 / 2.;

    let player_sprite_height = 64. * PLAYER_SCALE;

    for mut transform in player_query.iter_mut() {
        let player_center_y = transform.translation.y;
        let player_feet = player_center_y - (player_sprite_height / 2.);
        let t = ((player_feet - map_y0) / map_height).clamp(0., 1.);
        let player_z = PLAYER_BASE_Z + NODE_SIZE_Z * (1. - t) + PLAYER_Z_OFFSET;
        transform.translation.z = player_z;
    }
}
