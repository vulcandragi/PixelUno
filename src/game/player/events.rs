use bevy::{
    ecs::{
        observer::On,
        query::With,
        system::{Commands, Query, Res},
    },
    log::warn,
    picking::events::PointerClick,
    scene::CommandsSceneExt,
    state::{commands::CommandsStatesExt, state::State},
};

use crate::game::{
    card::components::{Card, CardSymbol},
    color_selector::spawn_color_select,
    game_manager::{components::GamePhase, events::ActionIntent},
    player::components::{PlayerTurn, PlayerType},
};

pub fn on_player_click_card(
    on: On<PointerClick>,
    mut commands: Commands,
    game_phase: Res<State<GamePhase>>,
    player: Query<&PlayerType, With<PlayerTurn>>,
    cards: Query<&Card>,
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

    let card_entity = on.original_event_target();
    let Ok(card) = cards.get(card_entity) else {
        return;
    };

    if card.symbol == CardSymbol::Plus4 || card.symbol == CardSymbol::Color {
        commands.set_state(GamePhase::WaitingColorSelect);
        commands.spawn_scene(spawn_color_select(card_entity));
        return;
    }

    commands.trigger(ActionIntent::Card {
        next_card: card_entity,
    });
}
