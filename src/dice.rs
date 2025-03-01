use crate::effect::Effect;
use bevy::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use rand::seq::SliceRandom;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Dice>()
        .register_type::<RolledEffect>()
        .add_observer(roll_dice)
        .add_observer(animate_dice);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Dice {
    pub sides: Vec<Handle<Effect>>,
}

/// This trigger rolls the dice and add `RolledEffect` component to the target entity.
#[derive(Event, Debug)]
pub struct Roll;

#[derive(Event, Debug)]
pub struct RollEnded;

/// This component is added to the dice when it is rolled and can be used to
/// play the animation.
#[derive(Component, Reflect, Debug)]
#[component(storage = "SparseSet")]
#[reflect(Component)]
pub struct RolledEffect(pub Handle<Effect>);

fn roll_dice(
    trigger: Trigger<Roll>,
    dice: Query<&Dice>,
    mut commands: Commands,
    entropy: GlobalEntropy<WyRand>,
) {
    let entity = trigger.entity();
    let Ok(dice) = dice.get(entity) else {
        return;
    };

    let mut entropy = entropy.into_inner();

    let Some(side) = dice.sides.choose(entropy.as_mut()) else {
        // Should never happen but I don't want to use `unwrap`
        error!("Dice has no sides!");
        return;
    };

    commands.entity(entity).insert(RolledEffect(side.clone()));
}

// TODO: Should we use `OnInsert` or `OnAdd` component here?
fn animate_dice(trigger: Trigger<OnInsert, RolledEffect>, mut commands: Commands) {
    info!("Animation can run now");
    commands.trigger_targets(RollEnded, [trigger.entity()]);
}
