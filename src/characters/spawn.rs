use std::path::MAIN_SEPARATOR;

use bevy::prelude::*;

use crate::{
    characters::{
        animation::{AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME},
        collider::Collider,
        config::{CharacterEntry, CharactersList},
        facing::Facing,
        input::Player,
        physics::Velocity,
        state::CharacterState,
    },
    collision::CollisionMap,
    combat::PlayerCombat,
    config::player::{COLLIDER_RADIUS, PLAYER_SCALE, PLAYER_Z_POSITION},
};

#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    pub index: usize,
}

#[derive(Resource)]
pub struct CharactersListResource {
    pub handle: Handle<CharactersList>,
}

pub fn create_character_atlas_layout(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    character_entry: &CharacterEntry,
) -> Handle<TextureAtlasLayout> {
    let max_row = character_entry.calculate_max_animation_row();

    atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ))
}

pub fn switch_character(
    input: Res<ButtonInput<KeyCode>>,
    mut character_index: ResMut<CurrentCharacterIndex>,
    characters_list: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<(&mut CharacterEntry, &mut Sprite), With<Player>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    const DIGIT_KEYS: [KeyCode; 9] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    let new_index = DIGIT_KEYS.iter().position(|&key| input.just_pressed(key));

    let Some(new_index) = new_index else {
        return;
    };

    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    let Some(characters_list) = characters_list.get(&characters_list_res.handle) else {
        return;
    };

    if new_index >= characters_list.characters.len() {
        return;
    }

    character_index.index = new_index;

    let Ok((mut current_entry, mut sprite)) = query.single_mut() else {
        return;
    };

    let character_entry = &characters_list.characters[new_index];

    *current_entry = character_entry.clone();

    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

    *sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct PlayerSpawned(pub bool);

fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    if collision_map.is_circle_clear(desired_pos, COLLIDER_RADIUS) {
        return desired_pos;
    }

    if let Some(clear_pos) = collision_map.find_nearest_clear_position(desired_pos, COLLIDER_RADIUS)
    {
        info!(
            "Adjusted player spawn from {:?} to {:?} (was an obstacle).",
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

pub fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut character_index: ResMut<CurrentCharacterIndex>,
) {
    let character_list_handle: Handle<CharactersList> =
        asset_server.load(format!("characters{MAIN_SEPARATOR}characters.ron"));

    commands.insert_resource(CharactersListResource {
        handle: character_list_handle,
    });

    character_index.index = 0;

    info!("Character assets loading started!");
}

pub fn spawn_player_at_valid_position(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_list: Res<Assets<CharactersList>>,
    character_index: Res<CurrentCharacterIndex>,
    characters_list_res: Option<Res<CharactersListResource>>,
    collision_map: Option<Res<CollisionMap>>,
    mut player_spawned: ResMut<PlayerSpawned>,
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

    if character_index.index >= characters_list.characters.len() {
        warn!("Invalid character index: {}", character_index.index);
        return;
    }

    let character_entry = &characters_list.characters[character_index.index];

    let desired_pos = Vec2::new(0., 0.);
    let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);

    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(valid_pos.x, valid_pos.y, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        sprite,
        AnimationController::default(),
        CharacterState::default(),
        Velocity::default(),
        Facing::default(),
        Collider::default(),
        PlayerCombat::default(),
        AnimationTimer(Timer::from_seconds(
            DEFAULT_ANIMATION_FRAME_TIME,
            TimerMode::Repeating,
        )),
        character_entry.clone(),
    ));

    player_spawned.0 = true;
    info!("Player spawned at validated position {:?}", valid_pos);
}
