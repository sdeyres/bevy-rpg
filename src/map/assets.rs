use std::path::MAIN_SEPARATOR;

use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

use crate::{
    collision::{TileMarker, TileType},
    map::tilemap::TILEMAP,
};

#[derive(Clone)]
pub struct SpawnableAsset {
    sprite_name: &'static str,
    grid_offset: GridDelta,
    offset: Vec3,
    tile_type: Option<TileType>,
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            tile_type: None,
        }
    }

    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }

    pub fn with_tile_type(mut self, tile_type: TileType) -> Self {
        self.tile_type = Some(tile_type);
        self
    }
}

#[derive(Clone)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

pub fn prepare_tilemap_handles(
    asset_server: Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image =
        asset_server.load::<Image>(format!("{assets_directory}{MAIN_SEPARATOR}{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    assets_definitions: Vec<Vec<SpawnableAsset>>,
) -> ModelsAssets<Sprite> {
    let mut models_assets = ModelsAssets::<Sprite>::new();

    for (model_index, assets) in assets_definitions.into_iter().enumerate() {
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                tile_type,
            } = asset_def;

            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'", sprite_name);
            };

            let spawner = create_spawner(tile_type);

            models_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    spawn_commands: spawner,
                    grid_offset,
                    world_offset: offset,
                },
            );
        }
    }

    models_assets
}

fn create_spawner(tile_type: Option<TileType>) -> fn(&mut EntityCommands) {
    match tile_type {
        Some(TileType::Dirt) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Dirt));
        },
        Some(TileType::Grass) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Grass));
        },
        Some(TileType::YellowGrass) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::YellowGrass));
        },
        Some(TileType::Shore) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Shore));
        },
        Some(TileType::Water) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Water));
        },
        Some(TileType::Tree) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Tree));
        },
        Some(TileType::Rock) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Rock));
        },
        Some(TileType::Empty) => |e: &mut EntityCommands| {
            e.insert(TileMarker::new(TileType::Empty));
        },
        _ => |_: &mut EntityCommands| {},
    }
}
