use bevy::{
    asset::Handle,
    ecs::{component::Component, entity::Entity, resource::Resource},
    image::Image,
    reflect::Reflect,
    time::Timer,
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Component, Clone, Default, Reflect)]
pub struct Player {
    pub cards: Vec<Entity>,
}

#[derive(Component, Clone, Default, Reflect)]
pub struct PlayerName;

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "images/player_menu_name.png")]
    pub menu_name: Handle<Image>,
}

#[derive(Component, Clone, Default, Reflect, PartialEq)]
pub enum PlayerType {
    #[default]
    Local,
    Pc {
        timer: Timer,
    },
}

#[derive(Component, Reflect)]
pub struct PlayerTurn;
