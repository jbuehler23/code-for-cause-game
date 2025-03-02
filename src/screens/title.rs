//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

use super::prototypes;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").observe(enter_gameplay);
            children.button("Credits").observe(enter_credits);

            #[cfg(feature = "dev")]
            children.button("Prototypes").observe(enter_prototypes);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });
}

fn enter_gameplay(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_credits(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

fn enter_prototypes(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_prototype_screen: ResMut<NextState<prototypes::PrototypesState>>,
) {
    next_screen.set(Screen::Prototypes);
    next_prototype_screen.set(prototypes::PrototypesState::Main);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
