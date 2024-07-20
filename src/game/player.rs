use crate::game::component::*;
use bevy::prelude::*;

use super::bundle::PlayerBundle;

pub fn spawn(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
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
