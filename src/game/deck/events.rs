use bevy::{
    ecs::{
        observer::On,
        system::{Commands, Res},
    },
    picking::events::PointerClick,
    state::state::State,
};

use crate::game::game_manager::{components::GamePhase, events::ActionIntent};

pub fn on_player_click_discard_pile(
    _: On<PointerClick>,
    mut commands: Commands,
    game_phase: Res<State<GamePhase>>,
) {
    if *game_phase.get() != GamePhase::WaitingForInput {
        return;
    }

    commands.trigger(ActionIntent::Buy);
}
