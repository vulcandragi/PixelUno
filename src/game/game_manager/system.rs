use bevy::{
    ecs::system::{Commands, ResMut},
    log::warn,
    state::commands::CommandsStatesExt,
};

use crate::game::{
    game_manager::components::{GameManager, GamePhase},
    player::components::PlayerTurn,
};

pub fn passing_turn(mut commands: Commands, mut game_manager: ResMut<GameManager>) {
    if let Some(current_player) = game_manager.current_player {
        commands.entity(current_player).remove::<PlayerTurn>();
    };

    let Some(current_player) = game_manager.advance_turn() else {
        warn!("Fail pass the turn");
        return;
    };

    commands.entity(current_player).insert(PlayerTurn);

    commands.set_state(GamePhase::WaitingForInput);
}
