pub mod player {
    pub const COLLIDER_RADIUS: f32 = 24.;

    pub const PLAYER_Z_POSITION: f32 = 20.;

    pub const PLAYER_SCALE: f32 = 1.2;
}

pub mod map {
    pub const TILE_SIZE: f32 = 64.;

    pub const GRID_X: u32 = 25;
    pub const GRID_Y: u32 = 18;

    pub const NODE_SIZE_Z: f32 = 1.;
}

pub mod pickup {
    pub const DEFAULT_RADIUS: f32 = 40.;
}

pub mod camera {
    pub const CAMERA_LERP_SPEED: f32 = 6.;
    pub const CAMERA_Z: f32 = 1000.;
}
