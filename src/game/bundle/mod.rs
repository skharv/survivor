use crate::{config::*, game::component};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: component::Player,
    pub direction: component::Direction,
    pub input_direction: component::InputDirection,
    pub movement: component::Movement,
    pub move_speed: component::MoveSpeed,
    pub turn_speed: component::TurnSpeed,
    pub velocity: component::Velocity,
    pub shape: ShapeBundle,
    pub fill: Fill,
    pub stroke: Stroke,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        let shape = shapes::RegularPolygon {
            sides: PLAYER_SIDES,
            feature: shapes::RegularPolygonFeature::Radius(PLAYER_SIZE),
            ..default()
        };

        PlayerBundle {
            player: component::Player,
            input_direction: component::InputDirection::default(),
            turn_speed: component::TurnSpeed {
                value: PLAYER_TURN_SPEED,
            },
            move_speed: component::MoveSpeed {
                value: PLAYER_MOVE_SPEED,
            },
            direction: component::Direction::default(),
            velocity: component::Velocity::default(),
            movement: component::Movement {
                style: component::MovementStyle::Pivot,
            },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(0., 0., PLAYER_LAYER)),
                ..default()
            },
            fill: Fill::color(PLAYER_FILL_COLOUR),
            stroke: Stroke::new(PLAYER_STROKE_COLOUR, PLAYER_STROKE_THICKNESS),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: component::Enemy,
    pub direction: component::Direction,
    pub input_direction: component::InputDirection,
    pub movement: component::Movement,
    pub move_speed: component::MoveSpeed,
    pub turn_speed: component::TurnSpeed,
    pub velocity: component::Velocity,
    pub shape: ShapeBundle,
    pub fill: Fill,
    pub stroke: Stroke,
}

impl Default for EnemyBundle {
    fn default() -> EnemyBundle {
        let shape = shapes::RegularPolygon {
            sides: ENEMY_SIDES,
            feature: shapes::RegularPolygonFeature::Radius(ENEMY_SIZE),
            ..default()
        };

        EnemyBundle {
            enemy: component::Enemy,
            input_direction: component::InputDirection::default(),
            turn_speed: component::TurnSpeed {
                value: ENEMY_TURN_SPEED,
            },
            move_speed: component::MoveSpeed {
                value: ENEMY_MOVE_SPEED,
            },
            direction: component::Direction::default(),
            velocity: component::Velocity::default(),
            movement: component::Movement {
                style: component::MovementStyle::FreeFormForward,
            },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    100.,
                    100.,
                    ENEMY_LAYER,
                )),
                ..default()
            },
            fill: Fill::color(ENEMY_FILL_COLOUR),
            stroke: Stroke::new(ENEMY_STROKE_COLOUR, ENEMY_STROKE_THICKNESS),
        }
    }
}
