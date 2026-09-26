use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(LoadState::Loading)
                .continue_to_state(LoadState::Complete)
                .on_failure_continue_to_state(LoadState::Error),
        )
        .add_systems(OnEnter(LoadState::Complete), load_complete)
        .add_message::<LoadCompleteMessage>()
        .insert_state(LoadState::Loading);
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum LoadState {
    None,
    Loading,
    Complete,
    Error,
}

#[derive(Message)]
pub struct LoadCompleteMessage;

fn load_complete(mut writer: MessageWriter<LoadCompleteMessage>) {
    writer.write(LoadCompleteMessage);
}
