use bevy::prelude::*;

use crate::GameAssets;

use super::{
    scrollable_list::{ScrollingIcon, ScrollingList, SelectorIcon},
    SpellBook, SpellbookViewDescription, SpellbookViewIcon, SpellbookViewTitle,
};

#[derive(Bundle)]
pub struct BackgroundBundle {
    image_bundle: ImageBundle,
}

impl BackgroundBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image_bundle: ImageBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_book_container.clone(),
                    ..default()
                },
                z_index: ZIndex::Local(-1),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MovementHintUpBundle {
    image_bundle: ImageBundle,
}

impl MovementHintUpBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image_bundle: ImageBundle {
                style: Style {
                    bottom: Val::Percent(53.0),
                    right: Val::Percent(105.0),
                    width: Val::Percent(10.0),
                    aspect_ratio: Some(0.5),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_book_hint_up.clone(),
                    ..default()
                },
                z_index: ZIndex::Local(-1),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MovementHintDownBundle {
    image_bundle: ImageBundle,
}

impl MovementHintDownBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image_bundle: ImageBundle {
                style: Style {
                    top: Val::Percent(53.0),
                    right: Val::Percent(105.0),
                    width: Val::Percent(10.0),
                    aspect_ratio: Some(0.5),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_book_hint_down.clone(),
                    ..default()
                },
                z_index: ZIndex::Local(-1),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewIconBundle {
    spellbook_view_icon: SpellbookViewIcon,
    image_bundle: ImageBundle,
}

impl Default for SpellbookViewIconBundle {
    fn default() -> Self {
        Self {
            spellbook_view_icon: SpellbookViewIcon,
            image_bundle: ImageBundle {
                style: Style {
                    top: Val::Percent(16.5),
                    left: Val::Percent(44.87),
                    width: Val::Percent(10.26),
                    height: Val::Percent(16.7),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewTitleBundle {
    spellbook_view_title: SpellbookViewTitle,
    text_bundle: TextBundle,
}

impl SpellbookViewTitleBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            spellbook_view_title: SpellbookViewTitle,
            text_bundle: TextBundle {
                text: Text::from_sections([TextSection {
                    value: "NO SPELL YET".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                }]),
                style: Style {
                    top: Val::Percent(45.0),
                    left: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewDescriptionBundle {
    spellbook_view_description: SpellbookViewDescription,
    text_bundle: TextBundle,
}

impl SpellbookViewDescriptionBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        let value = "Walk up to a statue and defeat all slimes.".to_string()
            + " You will get a new spell from each statue.\n"
            + "Press 'i' to open your spell console and type your spell. Try 'fireball'.";
        Self {
            spellbook_view_description: SpellbookViewDescription,
            text_bundle: TextBundle {
                text: Text::from_sections([TextSection {
                    value,
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                }]),
                style: Style {
                    top: Val::Percent(58.0),
                    left: Val::Percent(10.0),
                    width: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookViewBundle {
    image_bundle: ImageBundle,
}

impl SpellbookViewBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            image_bundle: ImageBundle {
                style: Style {
                    top: Val::Percent(20.0),
                    left: Val::Percent(107.0),
                    width: Val::Percent(110.0),
                    height: Val::Percent(60.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_book_view.clone(),
                    ..default()
                },
                z_index: ZIndex::Local(-1),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpellbookBundle {
    spellbook: SpellBook,
    node_bundle: NodeBundle,
}

impl Default for SpellbookBundle {
    fn default() -> Self {
        Self {
            spellbook: SpellBook,
            node_bundle: NodeBundle {
                style: Style {
                    height: Val::Percent(80.0),
                    width: Val::Percent(40.0),
                    top: Val::Percent(10.0),
                    left: Val::Percent(10.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(200),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MovingPanelBundle {
    scrolling_list: ScrollingList,
    node_bundle: NodeBundle,
}

impl Default for MovingPanelBundle {
    fn default() -> Self {
        Self {
            scrolling_list: ScrollingList { index: 0 },
            node_bundle: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(25.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MovingPanelLabelBundle {
    label: Label,
    image_bundle: ImageBundle,
}

impl MovingPanelLabelBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            label: Label,
            image_bundle: ImageBundle {
                style: Style {
                    width: Val::Px(78.0),
                    height: Val::Px(78.0),
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_field.clone(),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ScrollingIconBundle {
    scrolling_icon: ScrollingIcon,
    image_bundle: ImageBundle,
}

impl ScrollingIconBundle {
    pub fn new(texture: Handle<Image>, index: usize) -> Self {
        Self {
            scrolling_icon: ScrollingIcon { index },
            image_bundle: ImageBundle {
                style: Style {
                    top: Val::Px(7.0),
                    left: Val::Px(7.0),
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                image: UiImage {
                    texture,
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SelectorIconBundle {
    selector_icon: SelectorIcon,
    image_bundle: ImageBundle,
}

impl SelectorIconBundle {
    pub fn new(assets: &Res<GameAssets>) -> Self {
        Self {
            selector_icon: SelectorIcon,
            image_bundle: ImageBundle {
                style: Style {
                    top: Val::Px(-15.0),
                    left: Val::Px(-15.0),
                    width: Val::Px(94.0),
                    height: Val::Px(94.0),
                    ..default()
                },
                image: UiImage {
                    texture: assets.spell_field_selector.clone(),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ScrollableListBundle {
    node_bundle: NodeBundle,
}

impl Default for ScrollableListBundle {
    fn default() -> Self {
        Self {
            node_bundle: NodeBundle {
                style: Style {
                    top: Val::Percent(25.0),
                    height: Val::Percent(55.0),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            },
        }
    }
}
