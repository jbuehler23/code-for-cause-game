pub mod audio;
mod creature;
#[cfg(feature = "dev")]
mod dev_tools;
mod dialogue;
mod dice;
mod effect;
mod levels;
mod player;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use levels::{WINDOW_HEIGHT, WINDOW_WIDTH};
use screens::Screen;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        title: "Code For Cause Game".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Global RNG
        app.add_plugins(EntropyPlugin::<WyRand>::default());

        // Init the loading state for assets that can later be used with `configure_loading_state`.
        app.add_loading_state(LoadingState::new(Screen::Loading).continue_to_state(Screen::Title));

        // Add other plugins.
        app.add_plugins((
            screens::plugin,
            theme::plugin,
            effect::plugin,
            dice::plugin,
            dialogue::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

#[derive(thiserror::Error, Debug)]
pub enum AssetLoadingError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Ron(#[from] ron::error::SpannedError),
}
