mod camera;

use bevy::prelude::*;

pub use camera::MainCamera;

use crate::state::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera).add_systems(
            Update,
            camera::follow_player.run_if(in_state(GameState::Playing)),
        );
    }
}
