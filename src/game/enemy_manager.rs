use std::f32::consts::PI;

use super::bundle::EnemyBundle;
use super::component::MoveSpeed;
use super::component::SpawnTimer;
use super::component::TurnSpeed;
use crate::config::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

pub fn spawn(mut commands: Commands) {
    commands.spawn(SpawnTimer { countdown: 2.0 });
}

pub fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    window_query: Query<&Window>,
    camera_query: Query<&Transform, With<Camera>>,
    mut spawn_query: Query<&mut SpawnTimer>,
) {
    let Ok(mut spawner) = spawn_query.single_mut() else { return };

    spawner.countdown -= time.delta_secs();

    if spawner.countdown <= 0.0 {
        spawner.countdown = 2.0;

        let mut rng = thread_rng();
        let Ok(window) = window_query.single() else { return };
        let Ok(camera) = camera_query.single() else { return };

        let angle = rng.gen_range(0.0..PI * 2.0);
        let hyp = Vec2::new(0., 0.).distance(window.size()) / 2.;

        let size = rng.gen_range(ENEMY_SIZE - 10.0..ENEMY_SIZE + 10.0);
        let sides = rng.gen_range(ENEMY_SIDES - 4..ENEMY_SIDES + 5);
        let move_speed = ENEMY_MOVE_SPEED - size;
        let turn_speed = ENEMY_TURN_SPEED / size;

        let spawn_point = Vec3::new(
            camera.translation.x + (hyp * angle.cos()),
            camera.translation.y + (hyp * angle.sin()),
            ENEMY_LAYER - size,
        );

        let shape = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(size),
            ..default()
        };

        let fill_colour = rng.gen_range(0.0..360.0);

        commands.spawn(EnemyBundle {
            move_speed: MoveSpeed { value: move_speed },
            turn_speed: TurnSpeed { value: turn_speed },
            shape: ShapeBuilder::with(&shape)
                .fill(Fill::color(Hsla::new(fill_colour, 1.0, 0.5, 1.0)))
                .stroke(Stroke::new(
                    Hsla::new(fill_colour, 1.0, 0.2, 1.0),
                    ENEMY_SIZE / 10.0,
                ))
                .build(),
            transform: Transform::from_translation(spawn_point),
            ..default()
        });
    }
}
