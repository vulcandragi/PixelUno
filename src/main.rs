use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    camera::{Camera2d, OrthographicProjection, Projection, ScalingMode},
    ecs::system::Commands,
    image::ImagePlugin,
    state::app::AppExtStates,
    window::{Window, WindowPlugin},
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{game::GamePlugin, menu::MenuPlugin, plugins::debug::DebugPlugin, states::AppState};

mod game;
mod menu;
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
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
        )
        .add_plugins(DebugPlugin)
        .add_plugins((GamePlugin, MenuPlugin))
        .add_systems(Startup, setup)
        .run();
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
}
