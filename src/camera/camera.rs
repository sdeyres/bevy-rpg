use bevy::prelude::*;

use crate::{
    characters::input::Player,
    config::camera::{CAMERA_LERP_SPEED, CAMERA_Z},
};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));
}

pub fn follow_player(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let camera_pos = camera_transform.translation.truncate();

    let distance = player_pos.distance(camera_pos);
    if distance < 0.5 {
        return;
    }

    let lerp_factor = (CAMERA_LERP_SPEED * time.delta_secs()).clamp(0., 1.);
    let new_pos = camera_pos.lerp(player_pos, lerp_factor);

    camera_transform.translation.x = new_pos.x.round();
    camera_transform.translation.y = new_pos.y.round();
    camera_transform.translation.z = CAMERA_Z;
}
