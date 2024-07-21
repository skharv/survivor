use super::bundle::EnemyBundle;
use super::component::*;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    commands.spawn(EnemyBundle::default());
}

pub fn face_player(
    mut enemy_query: Query<(&mut InputDirection, &Transform), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (mut direction, enemy_transform) in enemy_query.iter_mut() {
        let input =
            (player_transform.translation.xy() - enemy_transform.translation.xy()).normalize();

        direction.x = input.x;
        direction.y = input.y;
    }
}
