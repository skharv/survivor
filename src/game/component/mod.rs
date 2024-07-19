use bevy::prelude::*;

#[derive(Component)]
pub struct InputDirection {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TurnSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct Direction {
    pub value: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct MoveSpeed {
    pub value: f32,
}
