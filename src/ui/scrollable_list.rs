use bevy::prelude::*;

use crate::{
    item::{item_value::item_icon, Item},
    GameAssets,
};

#[derive(Component)]
pub struct ScrollingList {
    index: usize,
    count: usize,
}

#[derive(Component)]
struct ScrollingIcon {
    index: usize,
}

#[derive(Component)]
struct SelectorIcon;

pub fn spawn_scrollable_list(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    spells: Vec<Item>,
) -> Entity {
    let moving_panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(25.0),
                    ..default()
                },
                ..default()
            },
            ScrollingList {
                index: 0,
                count: spells.len(),
            },
        ))
        .with_children(|parent| {
            for i in 0..spells.len() {
                parent
                    .spawn((
                        ImageBundle {
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
                        Label,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            ScrollingIcon { index: i },
                            ImageBundle {
                                style: Style {
                                    top: Val::Px(7.0),
                                    left: Val::Px(7.0),
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
                                    ..default()
                                },
                                image: UiImage {
                                    texture: item_icon(assets, &spells[i]),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    })
                    .with_children(|parent| {
                        if i == 0 {
                            parent.spawn((
                                SelectorIcon,
                                ImageBundle {
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
                            ));
                        }
                    });
            }
        })
        .id();

    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Percent(25.0),
                height: Val::Percent(55.0),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::clip_y(),
                ..default()
            },
            ..default()
        })
        .push_children(&[moving_panel])
        .id()
}

fn scroll_lists(
    keys: Res<Input<KeyCode>>,
    mut q_scrollable_lists: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    q_nodes: Query<&Node>,
) {
    for (mut scrolling_list, mut style, parent, list_node) in &mut q_scrollable_lists {
        let items_height = list_node.size().y;
        let container_height = match q_nodes.get(parent.get()) {
            Ok(n) => n.size().y,
            Err(_) => continue,
        };

        let max_scroll = (items_height - container_height).abs();

        if keys.just_pressed(KeyCode::J) {
            if scrolling_list.index != scrolling_list.count - 1 {
                scrolling_list.index += 1;
            }
        }
        if keys.just_pressed(KeyCode::K) {
            if scrolling_list.index != 0 {
                scrolling_list.index -= 1;
            }
        }

        let position = (-100.0 * scrolling_list.index as f32).clamp(-max_scroll, 0.0);
        style.top = Val::Px(position);
    }
}

fn update_selector_icon(
    mut commands: Commands,
    q_scrollable_lists: Query<&ScrollingList>,
    q_selector_icon: Query<Entity, With<SelectorIcon>>,
    q_scrollable_icons: Query<(Entity, &ScrollingIcon)>,
) {
    let scrolling_list = match q_scrollable_lists.get_single() {
        Ok(l) => l,
        Err(_) => return,
    };
    let selector_entity = match q_selector_icon.get_single() {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut icon = None;

    for (entity, scrollable_icon) in &q_scrollable_icons {
        if scrollable_icon.index == scrolling_list.index {
            icon = Some(entity);
            break;
        }
    }

    match icon {
        Some(icon) => {
            commands.entity(icon).push_children(&[selector_entity]);
        }
        None => {}
    };
}

pub struct ScrollableListPlugin;

impl Plugin for ScrollableListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (scroll_lists, update_selector_icon).chain());
    }
}
