use bevy::{
    ecs::{
        component::Component,
        hierarchy::Children,
        name::Name,
        observer::On,
        system::{Commands, Query, Single},
    },
    math::{Quat, Vec3},
    transform::components::Transform,
};
use rand::RngExt;

use crate::{
    events::Spawn,
    gameplay::{
        card::{AddCard, Card, CardSpawnData},
        deck::Deck,
    },
};

#[derive(Component)]
pub struct Stack {
    pub last_card: Option<Card>,
}

impl Stack {
    pub fn on_spawn(_: On<Spawn<Stack>>, mut commands: Commands, mut deck: Single<&mut Deck>) {
        let Some(card) = deck.get_next_card() else {
            return;
        };

        let entity = commands
            .spawn(Stack {
                last_card: Some(card.clone()),
            })
            .insert(Name("Stack".into()))
            .insert(Transform::default())
            .observe(Self::on_add_card)
            .id();

        commands.trigger(
            Spawn::<Card, CardSpawnData>::default()
                .with_data(CardSpawnData {
                    color: card.color,
                    symbol: card.symbol,
                })
                .with_parent(entity),
        );
    }

    fn on_add_card(
        event: On<AddCard>,
        mut query: Query<(&Card, &mut Transform)>,
        mut stack: Single<(&mut Stack, &Children)>,
    ) {
        let Ok((card, mut transform)) = query.get_mut(event.card) else {
            return;
        };

        let mut rng = rand::rng();

        transform.translation = Vec3 {
            x: rng.random_range(-10..10) as f32,
            y: rng.random_range(-10..10) as f32,
            z: stack.1.iter().count() as f32 + 1.,
        };
        transform.rotation = Quat::from_rotation_z(rng.random_range(-1.0..1.0));

        stack.0.last_card = Some(card.clone());
    }
}
