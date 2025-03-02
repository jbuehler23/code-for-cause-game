//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
#[cfg(feature = "dev")]
mod prototypes;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "dev"))]
    app.init_state::<Screen>();
    #[cfg(feature = "dev")]
    app.init_state::<Screen>()
        .init_state::<prototypes::PrototypesState>()
        .add_sub_state::<prototypes::PrototypesState>()
        .enable_state_scoped_entities::<prototypes::PrototypesState>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(prototypes::plugin);
}

/// The game's main screen states.
#[cfg(not(feature = "dev"))]
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
}

#[cfg(feature = "dev")]
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
    Prototypes,
}
