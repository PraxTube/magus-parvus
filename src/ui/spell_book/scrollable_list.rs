use bevy::prelude::*;

use crate::{
    item::{item_value::item_icon, ActiveItems},
    GameAssets,
};

use super::bundle::{
    MovingPanelBundle, MovingPanelLabelBundle, ScrollableListBundle, ScrollingIconBundle,
    SelectorIconBundle,
};

#[derive(Component)]
pub struct ScrollingList {
    pub index: usize,
}

#[derive(Component)]
pub struct ScrollingIcon {
    pub index: usize,
}

#[derive(Component)]
pub struct SelectorIcon;

const OFFSET: f32 = 25.0;
const INDEX_THRESHOLD: usize = 1;

pub fn spawn_scrollable_list(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    active_items: &Res<ActiveItems>,
) -> Entity {
    let moving_panel = commands
        .spawn(MovingPanelBundle::default())
        .with_children(|parent| {
            for (i, spell) in active_items.iter().enumerate() {
                parent
                    .spawn(MovingPanelLabelBundle::new(assets))
                    .with_children(|parent| {
                        parent.spawn(ScrollingIconBundle::new(item_icon(assets, spell), i));
                    })
                    .with_children(|parent| {
                        if i == 0 {
                            parent.spawn(SelectorIconBundle::new(assets));
                        }
                    });
            }
        })
        .id();

    commands
        .spawn(ScrollableListBundle::default())
        .push_children(&[moving_panel])
        .id()
}

fn scroll_lists(
    keys: Res<Input<KeyCode>>,
    active_items: Res<ActiveItems>,
    mut q_scrollable_lists: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    q_nodes: Query<&Node>,
) {
    let (mut scrolling_list, mut style, parent, list_node) =
        match q_scrollable_lists.get_single_mut() {
            Ok(s) => s,
            Err(_) => return,
        };
    let items_height = list_node.size().y;
    let container_height = match q_nodes.get(parent.get()) {
        Ok(n) => n.size().y,
        Err(_) => return,
    };

    if active_items.len() == 0 {
        return;
    }

    if (keys.just_pressed(KeyCode::J) || keys.just_pressed(KeyCode::S))
        && scrolling_list.index != active_items.len() - 1
    {
        scrolling_list.index += 1;
    }
    if (keys.just_pressed(KeyCode::K) || keys.just_pressed(KeyCode::W)) && scrolling_list.index != 0
    {
        scrolling_list.index -= 1;
    }

    let max_scroll = (items_height - container_height).abs() + 2.0 * OFFSET;
    let pos_index = if scrolling_list.index <= INDEX_THRESHOLD {
        0
    } else {
        scrolling_list.index - INDEX_THRESHOLD
    };
    let position = (-103.0 * pos_index as f32).clamp(-max_scroll, 0.0);
    style.top = Val::Px(position + OFFSET);
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

    if let Some(icon) = icon {
        commands.entity(icon).push_children(&[selector_entity]);
    };
}

pub struct ScrollableListPlugin;

impl Plugin for ScrollableListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (scroll_lists, update_selector_icon).chain());
    }
}
