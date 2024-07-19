use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
