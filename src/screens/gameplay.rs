//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    dice::{Dice, Roll, RollEnded, RolledEffect},
    effect::{ActionKind, Effect, EffectAction},
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.add_observer(update_value_text);

    app.add_systems(
        Update,
        // TODO: open pause menu instead of returning
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(
    mut commands: Commands,
    mut effects: ResMut<Assets<Effect>>,
    mut images: ResMut<Assets<Image>>,
) {
    let icon = images.add(Image::default());
    let dice = commands
        .spawn(Dice {
            sides: vec![
                effects.add(Effect {
                    icon: icon.clone(),
                    action: EffectAction {
                        kind: ActionKind::Damage,
                        value: 1,
                    },
                }),
                effects.add(Effect {
                    icon: icon.clone(),
                    action: EffectAction {
                        kind: ActionKind::Damage,
                        value: 78,
                    },
                }),
                effects.add(Effect {
                    icon: icon.clone(),
                    action: EffectAction {
                        kind: ActionKind::Heal,
                        value: 85,
                    },
                }),
                effects.add(Effect {
                    icon: icon.clone(),
                    action: EffectAction {
                        kind: ActionKind::Heal,
                        value: 1,
                    },
                }),
            ],
        })
        .id();

    commands
        .ui_root()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|children| {
            children
                .label("Rolled value: ")
                .with_child((TextSpan::new(""), RolledValueText));

            children.button("Roll").observe(
                move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger_targets(Roll, [dice]);
                },
            );
        });
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RolledValueText;

fn update_value_text(
    trigger: Trigger<RollEnded>,
    rolled_effects: Query<&RolledEffect>,
    mut text: Single<&mut TextSpan, With<RolledValueText>>,
    effects: Res<Assets<Effect>>,
) {
    let Some(rolled_effect) = rolled_effects
        .get(trigger.entity())
        .ok()
        .and_then(|e| effects.get(&e.0))
    else {
        return;
    };

    text.0 = format!(
        "{:?} {}",
        &rolled_effect.action.kind, &rolled_effect.action.value
    );
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
