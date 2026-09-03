use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Facing {
    Up,
    Left,
    #[default]
    Down,
    Right,
}

impl From<Vec2> for Facing {
    fn from(value: Vec2) -> Self {
        if value.x.abs() > value.y.abs() {
            if value.x > 0.0 {
                Self::Right
            } else {
                Self::Left
            }
        } else {
            if value.y > 0.0 { Self::Up } else { Self::Down }
        }
    }
}

impl Facing {
    pub(crate) fn direction_index(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Left => 1,
            Self::Down => 2,
            Self::Right => 3,
        }
    }
}

impl From<&Facing> for Vec3 {
    fn from(value: &Facing) -> Self {
        match value {
            Facing::Up => Vec3::Y,
            Facing::Left => Vec3::NEG_X,
            Facing::Down => Vec3::NEG_Y,
            Facing::Right => Vec3::X,
        }
    }
}
