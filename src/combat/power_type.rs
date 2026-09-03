use bevy::prelude::*;

use crate::particles::components::{EmissionShape, ParticleConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerType {
    #[default]
    Fire,
    Arcane,
    Shadow,
    Poison,
}

impl PowerType {
    pub fn visuals(&self, direction: Vec3) -> PowerVisuals {
        match self {
            Self::Fire => Self::fire_visuals(direction),
            Self::Arcane => Self::arcane_visuals(direction),
            Self::Shadow => Self::shadow_visuals(direction),
            Self::Poison => Self::poison_visuals(direction),
        }
    }

    fn fire_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.,
                lifetime_variance: 0.2,
                speed: 350.,
                speed_variance: 40.,
                direction,
                direction_variance: 0.12,
                scale: 1.5,
                scale_variance: 0.5,
                color: Color::srgb(3., 0.5, 0.1),
                angular_velocity: 3.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 10. },
            },
            core: Some(ParticleConfig {
                lifetime: 0.8,
                lifetime_variance: 0.2,
                speed: 350.,
                speed_variance: 30.,
                direction,
                direction_variance: 0.08,
                scale: 1.,
                scale_variance: 0.3,
                color: Color::srgb(4., 1., 0.2),
                angular_velocity: 5.,
                angular_velocity_variance: 2.,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 5. },
            }),
            particles_per_spawn: 5,
            core_particles_per_spawn: 3,
        }
    }

    fn arcane_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.2,
                lifetime_variance: 0.2,
                speed: 300.,
                speed_variance: 30.,
                direction,
                direction_variance: 0.05,
                scale: 1.2,
                scale_variance: 0.3,
                color: Color::srgb(0.5, 0.8, 2.5),
                angular_velocity: 2.0,
                angular_velocity_variance: 1.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 6. },
            },
            core: Some(ParticleConfig {
                lifetime: 1.0,
                lifetime_variance: 0.1,
                speed: 300.,
                speed_variance: 20.,
                direction,
                direction_variance: 0.03,
                scale: 0.8,
                scale_variance: 0.2,
                color: Color::srgb(0.9, 0.95, 3.0),
                angular_velocity: 0.5,
                angular_velocity_variance: 0.5,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 4,
            core_particles_per_spawn: 2,
        }
    }

    fn shadow_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 0.6,
                lifetime_variance: 0.15,
                speed: 600.,
                speed_variance: 100.,
                direction,
                direction_variance: 0.04,
                scale: 1.,
                scale_variance: 0.4,
                color: Color::srgb(0.6, 0.2, 1.2),
                angular_velocity: 8.0,
                angular_velocity_variance: 4.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            },
            core: Some(ParticleConfig {
                lifetime: 0.5,
                lifetime_variance: 0.1,
                speed: 650.,
                speed_variance: 80.,
                direction,
                direction_variance: 0.02,
                scale: 1.3,
                scale_variance: 0.3,
                color: Color::srgb(1., 0.5, 1.8),
                angular_velocity: 12.,
                angular_velocity_variance: 5.,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 7,
            core_particles_per_spawn: 3,
        }
    }

    fn poison_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.5,
                lifetime_variance: 0.4,
                speed: 200.,
                speed_variance: 50.,
                direction,
                direction_variance: 0.25,
                scale: 1.8,
                scale_variance: 0.6,
                color: Color::srgb(0.3, 2., 0.3),
                angular_velocity: 1.,
                angular_velocity_variance: 2.,
                acceleration: Vec3::new(0., 20., 0.),
                emission_shape: EmissionShape::Circle { radius: 15. },
            },
            core: None,
            particles_per_spawn: 6,
            core_particles_per_spawn: 0,
        }
    }
}

#[derive(Clone)]
pub struct PowerVisuals {
    pub primary: ParticleConfig,
    pub core: Option<ParticleConfig>,
    pub particles_per_spawn: u32,
    pub core_particles_per_spawn: u32,
}
