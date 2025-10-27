use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AssetState::Loading)
                .continue_to_state(AssetState::StartScreen)
                .on_failure_continue_to_state(AssetState::ErrorScreen)
                .load_collection::<CardAssets>(),
        )
        .insert_state(AssetState::SplashScreen);
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AssetState {
    SplashScreen,
    Loading,
    StartScreen,
    ErrorScreen,
}

#[derive(AssetCollection, Resource)]
pub struct CardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 84, tile_size_y = 120, columns = 15, rows = 5))]
    pub cards: Handle<TextureAtlasLayout>,
    #[asset(path = "images/cards.png")]
    pub cards_texture: Handle<Image>,
}
