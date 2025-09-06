use bevy::prelude::*;

use crate::GameAssets;

use super::{
    scrollable_list::{ScrollingIcon, ScrollingList, SelectorIcon},
    SpellBook, SpellbookViewDescription, SpellbookViewIcon, SpellbookViewTitle,
};

#[derive(Bundle)]
pub struct BackgroundBundle {
    image: ImageNode,
    style: Node,
    z_index: ZIndex,
}

impl BackgroundBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            style: Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: ImageNode::new(assets.spell_book_container.clone()),
            z_index: ZIndex(-1),
        }
    }
}

#[derive(Bundle)]
pub struct MovementHintUpBundle {
    image: ImageNode,
    style: Node,
    z_index: ZIndex,
}

impl MovementHintUpBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image: ImageNode::new(assets.spell_book_hint_up.clone()),
            style: Node {
                bottom: Val::Percent(53.0),
                right: Val::Percent(105.0),
                width: Val::Percent(10.0),
                aspect_ratio: Some(0.5),
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex(-1),
        }
    }
}

#[derive(Bundle)]
pub struct MovementHintDownBundle {
    image: ImageNode,
    style: Node,
    z_index: ZIndex,
}

impl MovementHintDownBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            style: Node {
                top: Val::Percent(53.0),
                right: Val::Percent(105.0),
                width: Val::Percent(10.0),
                aspect_ratio: Some(0.5),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: ImageNode::new(assets.spell_book_hint_down.clone()),
            z_index: ZIndex(-1),
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewIconBundle {
    spellbook_view_icon: SpellbookViewIcon,
    image: ImageNode,
    style: Node,
}

impl Default for SpellbookViewIconBundle {
    fn default() -> Self {
        Self {
            spellbook_view_icon: SpellbookViewIcon,
            image: ImageNode::default(),
            style: Node {
                top: Val::Percent(16.5),
                left: Val::Percent(44.87),
                width: Val::Percent(10.26),
                height: Val::Percent(16.7),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewTitleBundle {
    spellbook_view_title: SpellbookViewTitle,
    style: Node,
    text: Text,
    text_style: TextFont,
    text_color: TextColor,
}

impl SpellbookViewTitleBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            spellbook_view_title: SpellbookViewTitle,
            text: Text::from("NO SPELL YET"),
            text_style: TextFont {
                font: assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            text_color: TextColor(Color::WHITE),
            style: Node {
                top: Val::Percent(45.0),
                left: Val::Percent(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewDescriptionBundle {
    spellbook_view_description: SpellbookViewDescription,
    text: Text,
    text_font: TextFont,
    text_color: TextColor,
    text_style: Node,
}

impl SpellbookViewDescriptionBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        let value = "Walk up to a statue and defeat all slimes.".to_string()
            + " You will get a new spell from each statue.\n";
        Self {
            spellbook_view_description: SpellbookViewDescription,
            text: Text::from(value),
            text_font: TextFont {
                font: assets.font.clone(),
                font_size: 14.0,
                ..default()
            },
            text_color: TextColor(Color::WHITE),
            text_style: Node {
                top: Val::Percent(58.0),
                left: Val::Percent(10.0),
                width: Val::Percent(80.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewBundle {
    image: ImageNode,
    style: Node,
    z_index: ZIndex,
}

impl SpellbookViewBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image: ImageNode::new(assets.spell_book_view.clone()),
            style: Node {
                top: Val::Percent(20.0),
                left: Val::Percent(107.0),
                width: Val::Percent(110.0),
                height: Val::Percent(60.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex(-1),
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookBundle {
    spellbook: SpellBook,
    style: Node,
    z_index: ZIndex,
}

impl Default for SpellbookBundle {
    fn default() -> Self {
        Self {
            spellbook: SpellBook,
            style: Node {
                height: Val::Percent(80.0),
                width: Val::Percent(40.0),
                top: Val::Percent(10.0),
                left: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex(200),
        }
    }
}

#[derive(Bundle)]
pub struct MovingPanelBundle {
    scrolling_list: ScrollingList,
    style: Node,
}

impl Default for MovingPanelBundle {
    fn default() -> Self {
        Self {
            scrolling_list: ScrollingList { index: 0 },
            style: Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(25.0),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MovingPanelLabelBundle {
    label: Label,
    image: ImageNode,
    style: Node,
}

impl MovingPanelLabelBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            label: Label,
            style: Node {
                width: Val::Px(78.0),
                height: Val::Px(78.0),
                ..default()
            },
            image: ImageNode::new(assets.spell_field.clone()),
        }
    }
}

#[derive(Bundle)]
pub struct ScrollingIconBundle {
    scrolling_icon: ScrollingIcon,
    image: ImageNode,
    style: Node,
}

impl ScrollingIconBundle {
    pub fn new(texture: Handle<Image>, index: usize) -> Self {
        Self {
            scrolling_icon: ScrollingIcon { index },
            style: Node {
                top: Val::Px(7.0),
                left: Val::Px(7.0),
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default()
            },
            image: ImageNode::new(texture),
        }
    }
}

#[derive(Bundle)]
pub struct SelectorIconBundle {
    selector_icon: SelectorIcon,
    image: ImageNode,
    style: Node,
}

impl SelectorIconBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            selector_icon: SelectorIcon,
            style: Node {
                top: Val::Px(-15.0),
                left: Val::Px(-15.0),
                width: Val::Px(94.0),
                height: Val::Px(94.0),
                ..default()
            },
            image: ImageNode::new(assets.spell_field_selector.clone()),
        }
    }
}

#[derive(Bundle)]
pub struct ScrollableListBundle {
    style: Node,
}

impl Default for ScrollableListBundle {
    fn default() -> Self {
        Self {
            style: Node {
                top: Val::Percent(25.0),
                height: Val::Percent(55.0),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::clip_y(),
                ..default()
            },
        }
    }
}
