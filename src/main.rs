use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup},
    image::ImagePlugin,
    state::app::AppExtStates,
    window::{Window, WindowPlugin},
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{gameplay::GameplayPlugin, states::AppState};

mod events;
mod gameplay;
mod states;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pixel Uno".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Gameplay),
        )
        .add_plugins(GameplayPlugin)
        .run();
}
