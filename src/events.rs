use std::marker::PhantomData;

use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct Spawn<T, D = ()> {
    pub data: D,
    pub parent: Option<Entity>,
    _type: PhantomData<T>,
}

impl<T, D> Spawn<T, D> {
    pub fn with_data(mut self, data: D) -> Self {
        self.data = data;
        self
    }

    pub fn with_parent(mut self, parent: Entity) -> Self {
        self.parent = Some(parent);
        self
    }
}

impl<T, D: Default> Default for Spawn<T, D> {
    fn default() -> Self {
        Self {
            data: D::default(),
            parent: None,
            _type: PhantomData,
        }
    }
}
