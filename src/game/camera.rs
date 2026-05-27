use super::component::Player;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else { return };
    let Ok(player_transform) = player_query.single() else { return };

    camera_transform.translation = player_transform.translation;
}
