use super::component::Player;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation;
}
