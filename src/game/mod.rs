pub mod card;
pub mod components;
pub mod deck;
pub mod discard_pile;
pub mod player;
pub mod table;

use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Commands},
    scene::CommandsSceneExt,
    state::{app::AppExtStates, commands::CommandsStatesExt, condition::in_state, state::OnEnter},
};
use bevy_asset_loader::loading_state::{
    LoadingStateAppExt,
    config::{ConfigureLoadingState, LoadingStateConfig},
};

use crate::{
    game::{
        card::{components::CardAssets, systems::change_card_visibility},
        components::{GameManager, GamePhase},
        deck::{components::DeckAssets, spawn_deck, systems::distribute_cards},
        discard_pile::{spawn_discard_pile, systems::random_direction},
        player::systems::{orgazine_player_hand, register_players},
        table::{components::TableAssets, spawn_table},
    },
    states::AppState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading)
                .load_collection::<DeckAssets>()
                .load_collection::<CardAssets>()
                .load_collection::<TableAssets>(),
        )
        .init_state::<GamePhase>()
        .init_resource::<GameManager>()
        .add_systems(OnEnter(AppState::Game), setup)
        .add_systems(OnEnter(GamePhase::RegisterPlayers), register_players)
        .add_systems(OnEnter(GamePhase::DistributeCards), distribute_cards)
        .add_systems(
            Update,
            (
                orgazine_player_hand,
                change_card_visibility,
                random_direction,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(spawn_deck());
    commands.spawn_scene(spawn_table());
    commands.spawn_scene(spawn_discard_pile());
    commands.set_state(GamePhase::RegisterPlayers);
}
