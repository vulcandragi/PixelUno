use bevy::{
    ecs::{component::Component, entity::Entity},
    reflect::Reflect,
    time::Timer,
};

#[derive(Component, Clone, Default, Reflect)]
pub struct Player {
    pub cards: Vec<Entity>,
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
