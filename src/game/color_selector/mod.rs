use bevy::{
    color::{Color, palettes::tailwind::GRAY_800},
    ecs::{entity::Entity, hierarchy::Children},
    picking::Pickable,
    scene::{Scene, bsn, on},
    ui::{
        AlignItems, BackgroundColor, BorderRadius, CornerRadius, Display, FlexDirection, FlexWrap,
        JustifyContent, Node, PositionType, UiRect, percent, px,
    },
};

use crate::game::{
    card::components::CardColor,
    color_selector::{
        components::{ColorButton, ColorSelector},
        events::on_select_color,
    },
};

pub mod components;
pub mod events;

const BLUE_CARD: Color = Color::srgb_u8(121, 253, 255);
const YELLOW_CARD: Color = Color::srgb_u8(255, 238, 43);
const RED_CARD: Color = Color::srgb_u8(255, 94, 45);
const GREEN_CARD: Color = Color::srgb_u8(165, 255, 59);

pub fn spawn_color_select(card: Entity) -> impl Scene {
    let color_selector = ColorSelector(card);

    bsn! {
        #ColorSelector
        color_selector
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center
        }
        Children [
            Node {
                width: px(190),
                height: px(190),
                padding: UiRect::all(px(10)),
                border_radius: BorderRadius::all(CornerRadius::circular(px(10))),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                row_gap: px(10),
                column_gap: px(10)
            }
            BackgroundColor(GRAY_800)
            Children [
                Node {
                    width: px(80),
                    height: px(80)
                }
                BackgroundColor(BLUE_CARD)
                ColorButton(CardColor::Blue)
                Pickable
                --
                Node {
                    width: px(80),
                    height: px(80)
                }
                BackgroundColor(YELLOW_CARD)
                ColorButton(CardColor::Yellow)
                Pickable
                --
                Node {
                    width: px(80),
                    height: px(80)
                }
                BackgroundColor(RED_CARD)
                ColorButton(CardColor::Red)
                Pickable
                --
                Node {
                    width: px(80),
                    height: px(80)
                }
                BackgroundColor(GREEN_CARD)
                ColorButton(CardColor::Green)
                Pickable
            ]
        ]
        on(on_select_color)
    }
}
