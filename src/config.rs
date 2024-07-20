use bevy::prelude::*;

// Player
pub const PLAYER_SIZE: f32 = 50.0;
pub const PLAYER_FILL_COLOUR: Hsla = Hsla::hsl(180.0, 0.5, 0.5);
pub const PLAYER_STROKE_COLOUR: Hsla = Hsla::hsl(180.0, 1.0, 0.2);
pub const PLAYER_STROKE_THICKNESS: f32 = 3.0;
pub const PLAYER_TURN_SPEED: f32 = 10.0;
pub const PLAYER_MOVE_SPEED: f32 = 500.0;
pub const PLAYER_SIDES: usize = 4;
pub const PLAYER_LAYER: f32 = 10.0;

//Enemy
pub const ENEMY_SIZE: f32 = 20.0;
pub const ENEMY_FILL_COLOUR: Hsla = Hsla::hsl(0.0, 0.5, 0.5);
pub const ENEMY_STROKE_COLOUR: Hsla = Hsla::hsl(0.0, 1.0, 0.2);
pub const ENEMY_STROKE_THICKNESS: f32 = 2.0;
pub const ENEMY_TURN_SPEED: f32 = 10.0;
pub const ENEMY_MOVE_SPEED: f32 = 500.0;
pub const ENEMY_SIDES: usize = 7;
pub const ENEMY_LAYER: f32 = 5.0;
