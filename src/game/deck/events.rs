use bevy::{
    ecs::{
        observer::On,
        query::With,
        system::{Commands, Res, Single},
    },
    picking::events::PointerClick,
    state::state::State,
};

use crate::game::{
    game_manager::{components::GamePhase, events::ActionIntent},
    player::components::{Player, PlayerTurn, PlayerType},
};

pub fn on_player_click_discard_pile(
    _: On<PointerClick>,
    mut commands: Commands,
    game_phase: Res<State<GamePhase>>,
    player_type: Single<&PlayerType, (With<Player>, With<PlayerTurn>)>,
) {
    if *game_phase.get() != GamePhase::WaitingForInput {
        return;
    }

    if *player_type.into_inner() != PlayerType::Local {
        return;
    }

    commands.trigger(ActionIntent::Buy);
}
