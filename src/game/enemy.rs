use bevy::prelude::*;

use super::bundle::EnemyBundle;

pub fn spawn(mut commands: Commands) {
    commands.spawn(EnemyBundle::default());
}
