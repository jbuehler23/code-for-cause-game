use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PrototypesState::Main), setup);
    app.add_systems(OnEnter(PrototypesState::Battle), setup_battle_prototype);
    app.add_systems(
        Update,
        // TODO: open pause menu instead of returning
        return_to_main
            .run_if(in_state(Screen::Prototypes).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_sub_state::<PrototypesState>()
        .enable_state_scoped_entities::<PrototypesState>();
}

#[derive(Component, Debug)]
enum PrototypesButton {
    Battle,
    Dialog,
    Title,
}

#[derive(SubStates, Debug, PartialEq, Eq, Hash, Clone, Default)]
#[source(Screen = Screen::Prototypes)]
pub enum PrototypesState {
    #[default]
    Main,
    Battle,
    Dialog,
}

fn setup(mut commands: Commands) {
    let ui_root = commands
        .ui_root()
        .insert(StateScoped(PrototypesState::Main))
        .id();
    let header = commands.header("PROTOTYPES").id();
    let menu = commands
        .spawn(Node {
            display: Display::Flex,
            max_width: Val::Percent(70.),
            ..default()
        })
        .id();
    let battle_button = commands
        .button("BATTLE")
        .insert(PrototypesButton::Battle)
        .observe(on_button)
        .id();
    let dialog_button = commands
        .button("DIALOG")
        .insert(PrototypesButton::Dialog)
        .observe(on_button)
        .id();
    let title_button = commands
        .button("TITLE")
        .insert(PrototypesButton::Title)
        .observe(on_button)
        .id();

    commands.entity(ui_root).add_children(&[header, menu]);
    commands
        .entity(menu)
        .add_children(&[battle_button, dialog_button, title_button]);
}

fn on_button(
    trigger: Trigger<OnPress>,
    button_kind: Query<&PrototypesButton>,
    mut next_state: ResMut<NextState<PrototypesState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let entity = trigger.entity();
    if let Ok(button) = button_kind.get(entity) {
        match button {
            PrototypesButton::Battle => next_state.set(PrototypesState::Battle),
            PrototypesButton::Dialog => next_state.set(PrototypesState::Dialog),
            PrototypesButton::Title => {
                next_screen.set(Screen::Title);
            }
        }
    }
}

fn return_to_main(mut next_screen: ResMut<NextState<PrototypesState>>) {
    next_screen.set(PrototypesState::Main);
}

fn setup_battle_prototype(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("images/battle_prototype.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        StateScoped(PrototypesState::Battle),
    ));
}
