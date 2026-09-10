use bevy::prelude::*;

use crate::{
    characters::{Player, PlayerSpawned}, collision::{CollisionMap, CollisionMapBuilt, TileMarker}, combat::{HealthBarOwner, Projectile, ProjectileEffect}, enemy::{EnemiesSpawned, Enemy}, inventory::Inventory, map::{MapReady, WorldSeed}, network::PendingWorldSeed, particles::{Particle, ParticleEmitter}, state::GameState,
};

#[derive(Component)]
pub struct GameOverScreen;

pub fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0., 0., 0., 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER\n\nPress [R] to restart"),
                TextFont {
                    font_size: FontSize::Px(48.),
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::justify(Justify::Center),
            ));
        });

    info!("Game over screen spawned!");
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("Game over screen despawned!");
}

pub fn handle_restart_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        info!("Restarting game...");
        next_state.set(GameState::Playing);
    }
}

pub fn cleanup_game_world(
    mut commands: Commands,
    tiles: Query<Entity, With<TileMarker>>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    projectile_effects: Query<Entity, With<ProjectileEffect>>,
    emitters: Query<Entity, With<ParticleEmitter>>,
    particles: Query<Entity, With<Particle>>,
    healthbars: Query<Entity, With<HealthBarOwner>>,
    mut player_spawned: ResMut<PlayerSpawned>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
    mut collision_map_built: ResMut<CollisionMapBuilt>,
    mut inventory: ResMut<Inventory>,
) {
    for entity in tiles {
        commands.entity(entity).despawn();
    }
    for entity in players {
        commands.entity(entity).despawn();
    }
    for entity in &enemies {
        commands.entity(entity).despawn();
    }

    for entity in &projectiles {
        commands.entity(entity).despawn();
    }

    for entity in &projectile_effects {
        commands.entity(entity).despawn();
    }

    for entity in &emitters {
        commands.entity(entity).despawn();
    }

    for entity in &particles {
        commands.entity(entity).despawn();
    }

    for entity in &healthbars {
        commands.entity(entity).despawn();
    }

    player_spawned.0 = false;
    enemies_spawned.0 = false;
    collision_map_built.0 = false;
    commands.remove_resource::<CollisionMap>();
    inventory.set_items(Default::default());
    commands.remove_resource::<MapReady>();
    commands.remove_resource::<WorldSeed>();
    commands.remove_resource::<PendingWorldSeed>();
}
