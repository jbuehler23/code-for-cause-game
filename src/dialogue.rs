use bevy::prelude::*;

#[derive(Event)]
pub struct DialogueEvent {
    pub id: String,
    pub text: String,
}
pub(super) fn plugin(app: &mut App) {
    app.add_event::<DialogueEvent>();
}
