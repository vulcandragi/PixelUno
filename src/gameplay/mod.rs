pub mod card;
pub mod deck;
pub mod hand;
pub mod stack;

use bevy::{
    app::Plugin,
    camera::{Camera2d, ClearColor, OrthographicProjection, Projection, ScalingMode},
    color::{Color, Srgba},
    ecs::system::Commands,
    state::state::OnEnter,
};
use bevy_asset_loader::loading_state::{
    LoadingStateAppExt,
    config::{ConfigureLoadingState, LoadingStateConfig},
};

use crate::{
    events::Spawn,
    gameplay::{
        card::{Card, CardAssets},
        deck::{Deck, DeckAssets},
        hand::Hand,
        stack::Stack,
    },
    states::AppState,
};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading)
                .load_collection::<DeckAssets>()
                .load_collection::<CardAssets>(),
        )
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("9c6024").unwrap())))
        .add_systems(OnEnter(AppState::Gameplay), setup)
        .add_observer(Deck::on_spawn)
        .add_observer(Hand::on_spawn)
        .add_observer(Card::on_spawn)
        .add_observer(Stack::on_spawn);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 1280.,
                max_height: 720.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.trigger(Spawn::<Deck>::default());
    commands.trigger(Spawn::<Hand>::default());
    commands.trigger(Spawn::<Stack>::default());
}
