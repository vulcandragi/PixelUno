use bevy::{
    ecs::{
        observer::On,
        query::With,
        system::{Commands, Query, Res},
    },
    log::warn,
    picking::events::PointerClick,
    state::state::State,
};

use crate::game::{
    game_manager::{components::GamePhase, events::ActionIntent},
    player::components::{PlayerTurn, PlayerType},
};

pub fn on_player_click_card(
    on: On<PointerClick>,
    mut commands: Commands,
    game_phase: Res<State<GamePhase>>,
    player: Query<&PlayerType, With<PlayerTurn>>,
) {
    if *game_phase.get() != GamePhase::WaitingForInput {
        return;
    }

    let Ok(player_type) = player.get(on.entity) else {
        warn!("Player not found");
        return;
    };

    if *player_type != PlayerType::Local {
        return;
    }

    commands.trigger(ActionIntent::Card {
        next_card: on.original_event_target(),
    });
}
