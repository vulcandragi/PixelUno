use bevy::{
    ecs::{component::Component, entity::Entity, template::FromTemplate},
    reflect::Reflect,
};

#[derive(Component, FromTemplate, Clone, Reflect)]
pub struct Hand {
    pub owner: Entity,
    pub cards: Vec<Entity>,
}
