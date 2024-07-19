use crate::game::component::{self, *};
use bevy::prelude::*;

pub fn set_velocity(
    mut query: Query<(
        &mut Velocity,
        &mut InputDirection,
        &mut MoveSpeed,
        &mut component::Direction,
    )>,
) {
    for (mut velocity, input, move_speed, mut direction) in query.iter_mut() {
        if input.x == 0. && input.y == 0. {
            velocity.x = 0.;
            velocity.y = 0.;
        } else {
            direction.value = input.y.atan2(input.x);

            velocity.x = move_speed.value * f32::cos(direction.value);
            velocity.y = move_speed.value * f32::sin(direction.value);
        }
    }
}

pub fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn update_rotation(
    time: Res<Time>,
    mut query: Query<(&TurnSpeed, &mut Transform, &component::Direction)>,
) {
    for (turn_speed, mut transform, direction) in query.iter_mut() {
        let to_point = Vec3::new(
            transform.translation.x + direction.value.sin(),
            transform.translation.y - direction.value.cos(),
            0.,
        );

        face_point(
            &mut transform,
            turn_speed.value * time.delta_seconds(),
            to_point,
        );
    }
}

fn face_point(transform: &mut Transform, turn_amount: f32, point: Vec3) {
    let forward = (transform.rotation * Vec3::Y).truncate();
    let to_target = (point - transform.translation).truncate().normalize();
    let forward_dot_target = forward.dot(to_target);

    if (forward_dot_target - 1.0).abs() < f32::EPSILON {
        return;
    }

    let right = (transform.rotation * Vec3::X).truncate();
    let right_dot_target = right.dot(to_target);
    let rotation_sign = -f32::copysign(1.0, right_dot_target);

    let max_angle = forward_dot_target.clamp(-1.0, 1.0).acos();

    let rotation_angle = rotation_sign * turn_amount.min(max_angle);

    transform.rotate_z(rotation_angle);
}
