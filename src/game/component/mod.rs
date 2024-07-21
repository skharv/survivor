use bevy::prelude::*;

#[derive(Component)]
pub struct InputDirection {
    pub x: f32,
    pub y: f32,
}

impl Default for InputDirection {
    fn default() -> InputDirection {
        InputDirection { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Default)]
pub enum MovementStyle {
    #[default]
    Forward,
    FreeFormForward,
    Pivot,
    FreeFormPivot,
}

#[derive(Component)]
pub struct Movement {
    pub style: MovementStyle,
}

#[derive(Component)]
pub struct TurnSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct Direction {
    pub value: f32,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction { value: 0.0 }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct MoveSpeed {
    pub value: f32,
}
