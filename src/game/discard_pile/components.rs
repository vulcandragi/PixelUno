use bevy::{
    ecs::{component::Component, entity::Entity},
    reflect::Reflect,
};

#[derive(Component, Clone, Reflect, Default)]
pub struct DiscardPile {
    pub last_card: Option<Entity>,
}
