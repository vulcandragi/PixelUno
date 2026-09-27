use std::marker::PhantomData;

use bevy::ecs::event::Event;

#[derive(Event, Default)]
pub struct Spawn<T, D = ()> {
    pub data: D,
    _type: PhantomData<T>,
}
