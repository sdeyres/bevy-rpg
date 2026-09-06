use bevy::prelude::*;

use crate::{
    characters::input::Player,
    combat::spawn_projectile,
    enemy::components::{AIBehavior, Enemy, EnemyCombat},
};

pub fn enemy_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&GlobalTransform, &mut EnemyCombat, &AIBehavior), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (enemy_transform, mut combat, ai) in enemy_query.iter_mut() {
        combat.cooldown.tick(time.delta());

        let enemy_pos = enemy_transform.translation();
        let player_pos = player_transform.translation;

        let distance = enemy_pos.distance(player_pos);

        if distance <= ai.attack_range && combat.cooldown.elapsed() >= combat.cooldown.duration() {
            let to_player = (player_pos - enemy_pos).normalize();
            let spawn_position = enemy_pos + to_player * 5.;

            let visuals = combat.power_type.visuals(to_player);

            spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals);

            combat.cooldown.reset();

            info!("Enemy fired {:?} projectile at player!", combat.power_type);
        }
    }
}
