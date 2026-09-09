use bevy::prelude::*;

use crate::{
    characters::{
        AnimationController, AnimationTimer, CharacterState, CharactersList,
        CharactersListResource, Collider, DEFAULT_ANIMATION_FRAME_TIME, Facing, Velocity,
    },
    collision::CollisionMap,
    combat::Health,
    config::{
        enemy::{ENEMY_SCALE, ENEMY_Z_POSITION},
        player::COLLIDER_RADIUS,
    },
    enemy::components::{AIBehavior, Enemy, EnemyCombat, EnemyPath},
};

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    characters_list: &CharactersList,
    position: Vec3,
    character_name: &str,
) -> Option<Entity> {
    let character_entry = characters_list
        .characters
        .iter()
        .find(|c| c.name == character_name)?;

    let max_row = character_entry.calculate_max_animation_row();
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ));

    let texture = asset_server.load(&character_entry.texture_path);

    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    let entity = commands
        .spawn((
            Enemy,
            sprite,
            Transform::from_translation(position).with_scale(Vec3::splat(ENEMY_SCALE)),
            GlobalTransform::default(),
            AnimationController::default(),
            CharacterState::default(),
            Velocity::default(),
            Facing::default(),
            Collider::default(),
            EnemyCombat::default(),
            AIBehavior::default(),
            EnemyPath::default(),
            AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            Health::new(character_entry.max_health),
            character_entry.clone(),
        ))
        .id();

    info!("Spawned enemy '{}' at {:?}!", character_name, position);

    Some(entity)
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct EnemiesSpawned(pub bool);

fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    if collision_map.is_circle_clear(desired_pos, COLLIDER_RADIUS) {
        return desired_pos;
    }

    if let Some(clear_pos) = collision_map.find_nearest_clear_position(desired_pos, COLLIDER_RADIUS)
    {
        info!(
            "Adjusted spawn from {:?} to {:?} (was an obstacle)",
            desired_pos, clear_pos
        );
        return clear_pos;
    }

    warn!(
        "Could not find walkable spawn position near {:?}",
        desired_pos
    );
    desired_pos
}

pub fn spawn_test_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_list: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>,
    collision_map: Option<Res<CollisionMap>>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
) {
    let Some(collision_map) = collision_map else {
        return;
    };

    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    let Some(characters_list) = characters_list.get(&characters_list_res.handle) else {
        return;
    };

    let spawn_positions = [Vec2::new(200., 0.), Vec2::new(-200., 100.)];

    for desired_pos in spawn_positions {
        let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);

        spawn_enemy(
            &mut commands,
            &asset_server,
            &mut atlas_layouts,
            characters_list,
            Vec3::new(valid_pos.x, valid_pos.y, ENEMY_Z_POSITION),
            "graveyard_reaper",
        );
    }

    enemies_spawned.0 = true;
    info!("Enemies spawned with validated positions");
}
