use bevy::prelude::*;

use crate::AppState;

pub struct AStarPlugin;

impl Plugin for AStarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::AStar), // .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::AStar), // .with_system(menu_click_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::AStar), // .with_system(despawn_everything)
        );
    }
}
