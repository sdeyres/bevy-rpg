use bevy::prelude::*;

use crate::{
    audio::SfxKind,
    characters::{Facing, Player},
    combat::{
        events::ProjectileHit,
        player_combat::PlayerCombat,
        power_type::{PowerType, PowerVisuals},
    },
    enemy::Enemy,
    particles::ParticleEmitter,
};

#[derive(Component)]
pub struct ProjectileEffect {
    pub power_type: PowerType,
}

#[derive(Component, Clone, Copy)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub power_type: PowerType,
    pub owner: ProjectileOwner,
    pub radius: f32,
}

pub fn handle_power_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&GlobalTransform, &Facing, &mut PlayerCombat), With<Player>>,
) {
    let Ok((global_transform, facing, mut combat)) = player_query.single_mut() else {
        return;
    };

    combat.cooldown.tick(time.delta());

    let ctrl_pressed =
        input.just_pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight);

    if !ctrl_pressed {
        return;
    }

    if combat.cooldown.elapsed_secs() < combat.cooldown.duration().as_secs_f32() {
        return;
    }

    combat.cooldown.reset();

    let position = global_transform.translation();
    let direction: Vec3 = facing.into();
    let spawn_position = position + direction * 5.0;

    let visuals = combat.power_type.visuals(direction);

    spawn_projectile(
        &mut commands,
        spawn_position,
        combat.power_type,
        &visuals,
        ProjectileOwner::Player,
    );

    commands.trigger(SfxKind::PlayerShoot(combat.power_type));

    info!("{:?} projctile fired!", combat.power_type);
}

pub fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    power_type: PowerType,
    visuals: &PowerVisuals,
    owner: ProjectileOwner,
) {
    let primary_emitter =
        ParticleEmitter::new(0.016, visuals.particles_per_spawn, visuals.primary.clone())
            .one_shot();

    commands.spawn((
        primary_emitter,
        Transform::from_translation(position),
        GlobalTransform::from(Transform::from_translation(position)),
        ProjectileEffect { power_type },
    ));

    if let Some(ref core_config) = visuals.core {
        let core_emitter =
            ParticleEmitter::new(0.016, visuals.core_particles_per_spawn, core_config.clone())
                .one_shot();

        commands.spawn((
            core_emitter,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            ProjectileEffect { power_type },
        ));
    }

    let direction = visuals.primary.direction.normalize_or_zero();
    let speed = visuals.primary.speed;
    commands.spawn((
        Projectile {
            velocity: direction * speed,
            lifetime: 2.0,
            power_type,
            owner,
            radius: power_type.hitbox_radius(),
        },
        Transform::from_translation(position),
    ));
}

pub fn debug_switch_power(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut PlayerCombat, With<Player>>,
) {
    let Ok(mut combat) = player_query.single_mut() else {
        return;
    };

    let new_power = if input.just_pressed(KeyCode::Digit1) {
        Some(PowerType::Fire)
    } else if input.just_pressed(KeyCode::Digit2) {
        Some(PowerType::Arcane)
    } else if input.just_pressed(KeyCode::Digit3) {
        Some(PowerType::Shadow)
    } else if input.just_pressed(KeyCode::Digit4) {
        Some(PowerType::Poison)
    } else {
        None
    };

    if let Some(power) = new_power {
        combat.power_type = power;
        info!("Switched to {:?}", power);
    }
}

pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut projectile, mut transform) in projectiles.iter_mut() {
        projectile.lifetime -= dt;
        if projectile.lifetime <= 0. {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += projectile.velocity * dt;
    }
}

pub fn check_projectile_hits(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    players: Query<(Entity, &GlobalTransform), With<Player>>,
    enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
) {
    for (projectile_entity, projectile, transform) in &projectiles {
        let projectile_position = transform.translation;

        let hit_target = match projectile.owner {
            ProjectileOwner::Player => enemies
                .iter()
                .find(|(_, t)| projectile_position.distance(t.translation()) < projectile.radius)
                .map(|(e, _)| e),
            ProjectileOwner::Enemy => players
                .iter()
                .find(|(_, t)| projectile_position.distance(t.translation()) <= projectile.radius)
                .map(|(e, _)| e),
        };

        if let Some(target) = hit_target {
            commands.trigger(ProjectileHit {
                target,
                damage: projectile.power_type.damage(),
                power_type: projectile.power_type,
            });
            commands.entity(projectile_entity).despawn();
        }
    }
}
