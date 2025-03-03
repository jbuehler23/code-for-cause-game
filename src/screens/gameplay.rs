//! The screen state for the main gameplay.

use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{player::{camera_follow, player_movement, spawn_fog_of_war, spawn_player, update_fog_of_war, ExploredAreas}, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.insert_resource(ExploredAreas {
        visited: HashSet::new(),
    })
    .add_systems(OnEnter(Screen::Gameplay), (spawn_player, spawn_fog_of_war, player_movement, update_fog_of_war, camera_follow))

    .add_systems(
        Update,
        // TODO: open pause menu instead of returning
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
