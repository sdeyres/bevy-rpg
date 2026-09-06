use bevy::prelude::*;

use crate::{
    characters::{
        config::CharacterEntry, facing::Facing, input::Player, physics::{Velocity, calculate_velocity}, state::CharacterState,
    }, collision::CollisionMap, enemy::components::{AIBehavior, Enemy, EnemyPath},
};

pub fn enemy_follow_player(
    time: Res<Time>,
    collision_map: Option<Res<CollisionMap>>,
    mut enemy_query: Query<
        (
            &Transform,
            &mut CharacterState,
            &mut Velocity,
            &mut Facing,
            &CharacterEntry,
            &AIBehavior,
            &mut EnemyPath,
        ),
        With<Enemy>,
    >,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Some(collision_map) = collision_map else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let delta = time.delta_secs();

    for (enemy_transform, mut state, mut velocity, mut facing, character, ai, mut path) in
        enemy_query.iter_mut()
    {
        let enemy_pos = enemy_transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();

        if distance > ai.detection_range {
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            *velocity = Velocity::ZERO;
            continue;
        }

        let attack_threshold = if *state == CharacterState::Idle {
            ai.attack_range + 20.
        } else {
            ai.attack_range
        };

        if distance <= attack_threshold {
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            *velocity = Velocity::ZERO;

            let direction = to_player.normalize_or_zero();
            if direction != Vec2::ZERO {
                let new_facing = direction.into();
                if *facing != new_facing {
                    *facing = new_facing;
                }
            }
            continue;
        }

        path.recalc_timer -= delta;

        if !path.has_path() {
            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
                path.recalc_timer = EnemyPath::RECALC_INTERVAL;
            }
        } else if path.recalc_timer <= 0.0 {
            path.recalc_timer = EnemyPath::RECALC_INTERVAL;

            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
            }
        }

        if let Some(waypoint) = path.current_waypoint() {
            let to_waypoint = waypoint - enemy_pos;
            let waypoint_distance = to_waypoint.length();

            if waypoint_distance < EnemyPath::WAYPOINT_THRESHOLD {
                path.advance();
            }

            if let Some(current_wp) = path.current_waypoint() {
                let to_waypoint = current_wp - enemy_pos;
                let direction = to_waypoint.normalize_or_zero();

                if *state != CharacterState::Walking {
                    *state = CharacterState::Walking;
                }

                if direction != Vec2::ZERO {
                    let new_facing = direction.into();
                    if *facing != new_facing {
                        *facing = new_facing;
                    }
                }

                *velocity = calculate_velocity(*state, direction, character);
            }
        } else {
            let direction = to_player.normalize_or_zero();

            if *state != CharacterState::Walking {
                *state = CharacterState::Walking;
            }

            if direction != Vec2::ZERO {
                let new_facing = direction.into();
                if *facing != new_facing {
                    *facing = new_facing;
                }
            }

            *velocity = calculate_velocity(*state, direction, character);
        }
    }
}
