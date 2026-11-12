use bevy::{
    ecs::{
        query::{Changed, With},
        system::Query,
    },
    sprite::Sprite,
};

use crate::game::card::components::{Card, CardVisibility};

type ChangeCardVisibilityCardQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Card, &'static CardVisibility, &'static mut Sprite),
    (With<Card>, Changed<CardVisibility>),
>;

pub fn change_card_visibility(mut card: ChangeCardVisibilityCardQuery) {
    for (card, card_visibility, mut sprite) in card.iter_mut() {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        texture_atlas.index = if *card_visibility == CardVisibility::FaceUp {
            card.get_index()
        } else {
            62
        };
    }
}
