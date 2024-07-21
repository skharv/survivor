use crate::AppState;
use bevy::prelude::*;

pub fn go_ingame(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InGame);
}
