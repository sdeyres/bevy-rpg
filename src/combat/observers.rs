use bevy::prelude::*;

use crate::{
    audio::SfxKind,
    characters::Player,
    combat::{
        events::{EntityDeath, ProjectileHit},
        health::Health,
    },
    state::GameState,
};

pub fn on_projectile_hit(
    hit: On<ProjectileHit>,
    mut healths: Query<&mut Health>,
    mut commands: Commands,
) {
    let Ok(mut health) = healths.get_mut(hit.target) else {
        return;
    };

    health.take_damage(&mut commands, hit.target, hit.damage);

    commands.trigger(SfxKind::Hit);

    info!(
        "{:?} hit for {} damage! HP: {:.0} / {:.0}",
        hit.power_type, hit.damage, health.current, health.max
    );
}

pub fn on_entity_death(
    death: On<EntityDeath>,
    mut commands: Commands,
    players: Query<(), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let entity = death.entity;
    let is_player = players.get(entity).is_ok();

    info!("Entity {:?} defeated!", entity);
    commands.entity(entity).despawn();

    if is_player {
        info!("Player defeated! Game over...");
        commands.trigger(SfxKind::PlayerDeath);
        next_state.set(GameState::GameOver);
    } else {
        commands.trigger(SfxKind::EnemyDeath);
    }
}
