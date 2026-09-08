pub mod player {
    pub const COLLIDER_RADIUS: f32 = 24.;

    pub const PLAYER_Z_POSITION: f32 = 20.;

    pub const PLAYER_SCALE: f32 = 1.2;
}

pub mod enemy {
    pub const ENEMY_Z_POSITION: f32 = 20.;

    pub const ENEMY_SCALE: f32 = 1.2;
}

pub mod map {
    pub const TILE_SIZE: f32 = 64.;

    pub const GRID_X: u32 = 25;
    pub const GRID_Y: u32 = 18;

    pub const CHUNKS_X: u32 = 10;
    pub const CHUNKS_Y: u32 = 10;

    pub const TOTAL_GRID_X: u32 = CHUNKS_X * GRID_X - (CHUNKS_X - 1);
    pub const TOTAL_GRID_Y: u32 = CHUNKS_Y * GRID_Y - (CHUNKS_Y - 1);

    pub const NODE_SIZE_Z: f32 = 1.;
}

pub mod pickup {
    pub const DEFAULT_RADIUS: f32 = 40.;
}

pub mod camera {
    pub const CAMERA_LERP_SPEED: f32 = 6.;
    pub const CAMERA_Z: f32 = 1000.;
}

pub mod save {
    pub const SAVE_VERSION: u32 = 1;
    pub const MAX_SLOTS: usize = 5;
}
