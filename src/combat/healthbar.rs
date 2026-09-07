use bevy::prelude::*;

use crate::combat::health::Health;

const HEALTHBAR_WIDTH: f32 = 50.;
const HEALTHBAR_HEIGHT: f32 = 6.;
const HEALTHBAR_OFFSET_Y: f32 = 43.;
const HEALTHBAR_OFFSET_Z: f32 = 1.;
const HEALTHBAR_FG_Z_BUMP: f32 = 0.01;

#[derive(Component)]
pub struct HealthBarForeground;

#[derive(Component)]
pub struct HealthBarOwner(pub Entity);

pub fn spawn_healthbars(
    mut commands: Commands,
    new_health: Query<(Entity, &GlobalTransform, &Health), Added<Health>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (owner, transform, health) in &new_health {
        let pos = transform.translation();
        let bg_pos = Vec3::new(
            pos.x,
            pos.y + HEALTHBAR_OFFSET_Y,
            pos.z + HEALTHBAR_OFFSET_Z,
        );
        let fg_pos = Vec3::new(
            pos.x,
            pos.y + HEALTHBAR_OFFSET_Y,
            pos.z + HEALTHBAR_OFFSET_Z + HEALTHBAR_FG_Z_BUMP,
        );

        let bg_mesh = meshes.add(Rectangle::new(HEALTHBAR_WIDTH, HEALTHBAR_HEIGHT));
        let bg_mat = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.2, 0.2)));
        commands.spawn((
            Mesh2d(bg_mesh),
            MeshMaterial2d(bg_mat),
            Transform::from_translation(bg_pos),
            HealthBarOwner(owner),
        ));

        let ratio = health.ratio();
        let fg_mesh = meshes.add(Rectangle::new(HEALTHBAR_WIDTH, HEALTHBAR_HEIGHT));
        let fg_mat = materials.add(ColorMaterial::from(health_color(ratio)));
        commands.spawn((
            Mesh2d(fg_mesh),
            MeshMaterial2d(fg_mat),
            Transform::from_translation(fg_pos).with_scale(Vec3::new(ratio.max(0.001), 1.0, 1.0)),
            HealthBarOwner(owner),
            HealthBarForeground,
        ));
    }
}

pub fn update_healthbars(
    mut commands: Commands,
    mut bars: Query<(
        Entity,
        &HealthBarOwner,
        &mut Transform,
        Has<HealthBarForeground>,
        &MeshMaterial2d<ColorMaterial>,
    )>,
    owners: Query<(&GlobalTransform, &Health)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bar_entity, owner_ref, mut transform, is_foreground, mat_handle) in &mut bars {
        let Ok((owner_transform, health)) = owners.get(owner_ref.0) else {
            commands.entity(bar_entity).despawn();
            continue;
        };

        let owner_pos = owner_transform.translation();
        let ratio = health.ratio();

        if is_foreground {
            transform.scale.x = ratio.max(0.001);

            transform.translation = Vec3::new(
                owner_pos.x - (HEALTHBAR_WIDTH * (1. - ratio) / 2.),
                owner_pos.y + HEALTHBAR_OFFSET_Y,
                owner_pos.z + HEALTHBAR_OFFSET_Y + HEALTHBAR_FG_Z_BUMP,
            );

            if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
                mat.color = health_color(ratio);
            }
        } else {
            transform.translation = Vec3::new(
                owner_pos.x,
                owner_pos.y + HEALTHBAR_OFFSET_Y,
                owner_pos.z + HEALTHBAR_OFFSET_Z,
            );
        }
    }
}

fn health_color(ratio: f32) -> Color {
    if ratio >= 0.5 {
        let t = (ratio - 0.5) * 2.;
        Color::srgb(1.0 - t * 0.8, 0.8, 0.2)
    } else {
        let t = ratio * 2.;
        Color::srgb(1.0, t * 0.8, 0.2)
    }
}
