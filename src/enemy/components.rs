use bevy::prelude::*;

use crate::combat::PowerType;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyCombat {
    pub power_type: PowerType,
    pub cooldown: Timer,
}

impl Default for EnemyCombat {
    fn default() -> Self {
        Self {
            power_type: PowerType::Shadow,
            cooldown: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}

impl EnemyCombat {
    pub fn new(power_type: PowerType, cooldown_seconds: f32) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(cooldown_seconds, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct AIBehavior {
    pub attack_range: f32,
    pub detection_range: f32,
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            attack_range: 150.,
            detection_range: 500.,
        }
    }
}

impl AIBehavior {
    pub fn new(attack_range: f32, detection_range: f32) -> Self {
        Self {
            attack_range,
            detection_range,
        }
    }
}

#[derive(Component, Default)]
pub struct EnemyPath {
    pub waypoints: Vec<Vec2>,
    pub current_index: usize,
    pub recalc_timer: f32,
}

impl EnemyPath {
    pub const WAYPOINT_THRESHOLD: f32 = 16.0;
    pub const RECALC_INTERVAL: f32 = 0.5;

    pub fn current_waypoint(&self) -> Option<Vec2> {
        self.waypoints.get(self.current_index).copied()
    }

    pub fn advance(&mut self) -> bool {
        self.current_index += 1;
        self.current_index >= self.waypoints.len()
    }

    pub fn set_path(&mut self, waypoints: Vec<Vec2>) {
        let new_waypoints = if waypoints.len() > 1 {
            waypoints[1..].to_vec()
        } else {
            waypoints
        };

        if let Some(current_target) = self.current_waypoint() {
            if let Some(new_first) = new_waypoints.first() {
                if current_target.distance(*new_first) < Self::WAYPOINT_THRESHOLD * 1.5 {
                    return;
                }
            }
        }

        self.waypoints = new_waypoints;
        self.current_index = 0;
    }

    pub fn has_path(&self) -> bool {
        !self.waypoints.is_empty() && self.current_index < self.waypoints.len()
    }
}
