use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod bundle;
mod camera;
mod component;
mod dust;
mod enemy;
mod movement;
mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        app.add_systems(Startup, (camera::spawn, player::spawn, enemy::spawn));
        app.add_systems(
            Update,
            (
                player::set_direction.before(movement::set_velocity),
                enemy::face_player.before(movement::set_velocity),
                movement::set_velocity,
                movement::update_rotation.after(movement::set_velocity),
                movement::update_position.after(movement::update_rotation),
            ),
        );
    }
}
