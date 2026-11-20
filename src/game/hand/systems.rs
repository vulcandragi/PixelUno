use bevy::{ecs::{query::Changed, system::Query}, math::Vec3, transform::components::Transform};

use crate::game::{card::components::Card, hand::components::Hand, player::components::PlayerType};

pub fn orgazine_player_hand(
    hands: Query<&Hand, Changed<Hand>>,
    players: Query<&PlayerType>,
    mut cards: Query<(&Card, &mut Transform)>,
) {
    for hand in hands.iter() {
        let Ok(PlayerType::Local) = players.get(hand.owner) else {
            continue;
        };

        let card_count = hand.cards.len();
        let off_set = ((card_count as f32 * 60.) / 2.) + 30.;

        let mut sorted_cards = hand.cards.to_vec();

        sorted_cards
            .sort_by_key(|entity| cards.get(*entity).map_or(0, |(card, _)| card.get_index()));

        for (index, card_entity) in sorted_cards.iter().enumerate() {
            let Ok((card, mut transform)) = cards.get_mut(*card_entity) else {
                continue;
            };

            transform.translation = Vec3 {
                x: (-off_set) + (60. * (index + 1) as f32),
                z: card.get_index() as f32,
                ..Default::default()
            };
        }
    }
}
