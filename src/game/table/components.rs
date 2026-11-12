use bevy::{
    asset::Handle,
    ecs::{component::Component, resource::Resource},
    image::Image,
    reflect::Reflect,
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Component, Clone, Default, Reflect)]
pub struct Table;

#[derive(Resource, AssetCollection)]
pub struct TableAssets {
    #[asset(path = "images/background.webp")]
    pub image: Handle<Image>,
}
