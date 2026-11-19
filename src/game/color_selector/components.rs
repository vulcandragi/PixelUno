use bevy::{
    ecs::{component::Component, entity::Entity},
    reflect::Reflect,
};

use crate::game::card::components::CardColor;

#[derive(Component, Clone, Reflect)]
pub struct ColorSelector(pub Entity);

#[derive(Component, Clone, Default, Reflect)]
pub struct ColorButton(pub CardColor);
