use bevy::prelude::*;

use crate::{
    characters::state::CharacterState,
    config::{
        map::{NODE_SIZE_Z, TILE_SIZE, TOTAL_GRID_Y},
        player::PLAYER_SCALE,
    },
};

const CHARACTER_BASE_Z: f32 = 4.;
const CHARACTER_Z_OFFSET: f32 = 0.5;

pub fn update_character_depth(
    mut character_query: Query<&mut Transform, (With<CharacterState>, Changed<Transform>)>,
) {
    let map_height = TILE_SIZE * TOTAL_GRID_Y as f32;
    let map_y0 = -TILE_SIZE * TOTAL_GRID_Y as f32 / 2.;

    let character_sprite_height = 64. * PLAYER_SCALE;

    for mut transform in character_query.iter_mut() {
        let character_center_y = transform.translation.y;
        let character_feet_y = character_center_y - (character_sprite_height / 2.);
        let t = ((character_feet_y - map_y0) / map_height).clamp(0., 1.);
        let character_z = CHARACTER_BASE_Z + NODE_SIZE_Z * (1. - t) + CHARACTER_Z_OFFSET;
        transform.translation.z = character_z;
    }
}
