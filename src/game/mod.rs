use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod component;
mod movement;
mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        app.add_systems(Startup, (camera::spawn, player::spawn));
        app.add_systems(
            Update,
            (
                player::set_direction,
                movement::set_velocity.after(player::set_direction),
                movement::update_rotation.after(movement::set_velocity),
                movement::update_position.after(movement::update_rotation),
            ),
        );
    }
}
