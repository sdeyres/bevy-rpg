use std::collections::{HashMap, hash_map::Entry};

use bevy::prelude::*;

use crate::{
    collision::{
        map::CollisionMap,
        tile_type::{TileMarker, TileType},
    },
    map::generate::{GRID_X, GRID_Y, TILE_SIZE},
};

#[derive(Resource, Default, PartialEq, Eq)]
pub struct CollisionMapBuilt(pub bool);

pub fn build_collision_map(
    mut commands: Commands,
    mut built: ResMut<CollisionMapBuilt>,
    tile_query: Query<(&TileMarker, &Transform)>,
) {
    let mut tile_iter = tile_query.iter();
    let Some((first_marker, first_transform)) = tile_iter.next() else {
        return;
    };

    let grid_origin_x = -TILE_SIZE * GRID_X as f32 / 2.0;
    let grid_origin_y = -TILE_SIZE * GRID_Y as f32 / 2.0;

    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    let mut layer_tracker: HashMap<(i32, i32), (TileType, f32)> = HashMap::new();
    let mut tile_count: usize = 0;

    let mut process_tile = |marker: &TileMarker, transform: &Transform| {
        tile_count += 1;

        let world_x = transform.translation.x;
        let world_y = transform.translation.y;
        let world_z = transform.translation.z;

        let grid_x = ((world_x - grid_origin_x) / TILE_SIZE).floor() as i32;
        let grid_y = ((world_y - grid_origin_y) / TILE_SIZE).floor() as i32;

        min_x = min_x.min(grid_x);
        max_x = max_x.max(grid_x);
        min_y = min_y.min(grid_y);
        max_y = max_y.max(grid_y);

        match layer_tracker.entry((grid_x, grid_y)) {
            Entry::Occupied(mut entry) => {
                if world_z > entry.get().1 {
                    *entry.get_mut() = (marker.tile_type, world_z);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert((marker.tile_type, world_z));
            }
        }
    };

    process_tile(first_marker, first_transform);
    for (marker, transform) in tile_iter {
        process_tile(marker, transform);
    }

    let actual_width = (max_x - min_x + 1) as i32;
    let actual_height = (max_y - min_y + 1) as i32;

    let mut map = CollisionMap::new(
        actual_width,
        actual_height,
        TILE_SIZE,
        grid_origin_x,
        grid_origin_y,
    );

    for ((grid_x, grid_y), (tile_type, _z)) in layer_tracker.iter() {
        let local_x = grid_x - min_x;
        let local_y = grid_y - min_y;
        map.set_tile(local_x, local_y, *tile_type);
    }

    convert_water_edges_to_shore(&mut map);

    commands.insert_resource(map);
    built.0 = true;
}

fn convert_water_edges_to_shore(map: &mut CollisionMap) {
    let mut shores = Vec::new();

    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get_tile(x, y) != Some(TileType::Water) {
                continue;
            }

            let neighbors = [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ];

            for (nx, ny) in neighbors {
                if map.is_walkable(nx, ny) {
                    shores.push((x, y));
                    break;
                }
            }
        }
    }

    for (x, y) in shores {
        map.set_tile(x, y, TileType::Shore);
    }
}
