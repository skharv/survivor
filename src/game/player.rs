use crate::{
    config::*,
    game::component::{self, *},
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: PLAYER_SIDES,
        feature: shapes::RegularPolygonFeature::Radius(PLAYER_SIZE),
        ..default()
    };

    commands.spawn((
        Player,
        InputDirection { x: 0., y: 0. },
        TurnSpeed {
            value: PLAYER_TURN_SPEED,
        },
        MoveSpeed {
            value: PLAYER_MOVE_SPEED,
        },
        component::Direction { value: 0. },
        Velocity { x: 0., y: 0. },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(PLAYER_FILL_COLOUR),
        Stroke::new(PLAYER_STROKE_COLOUR, PLAYER_STROKE_THICKNESS),
    ));
}

pub fn set_direction(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputDirection, With<Player>>,
) {
    let mut direction = query.single_mut();
    direction.x = 0.;
    direction.y = 0.;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
}
