use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup},
    image::ImagePlugin,
    state::app::AppExtStates,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{gameplay::GameplayPlugin, plugins::debug::DebugPlugin, states::AppState};

mod events;
mod gameplay;
mod plugins;
mod states;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pixel Uno".into(),
                        resolution: WindowResolution::new(1280, 720),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Gameplay),
        )
        .add_plugins(DebugPlugin)
        .add_plugins(GameplayPlugin)
        .run();
}
