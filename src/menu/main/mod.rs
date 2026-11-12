use bevy::{
    app::Plugin,
    ecs::system::Commands,
    state::{commands::CommandsStatesExt, state::OnEnter},
};

use crate::states::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup);
    }
}

fn setup(mut commands: Commands) {
    commands.set_state(AppState::Game);
}
