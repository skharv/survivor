#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod config;
mod game;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Start,
    RoundStart,
    RoundEnd,
    InGame,
    Pause,
    Win,
    Loss,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            game::GamePlugin,
        ))
        .init_state::<AppState>()
        .run();
}
