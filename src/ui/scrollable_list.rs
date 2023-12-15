use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn spawn_scrollable_list(commands: &mut Commands) -> Entity {
    let moving_panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ScrollingList::default(),
        ))
        .with_children(|parent| {
            for i in 0..30 {
                parent.spawn((
                    TextBundle::from_section(
                        format!("Item {i}"),
                        TextStyle {
                            font_size: 40.,
                            ..default()
                        },
                    ),
                    Label,
                ));
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

        let max_scroll = (items_height - container_height).max(0.);
        let mut dy = 0.0;
        if keys.just_pressed(KeyCode::J) {
            dy -= 1.0;
        }
        if keys.just_pressed(KeyCode::K) {
            dy += 1.0;
        }

        scrolling_list.position += 40.0 * dy;
        scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
        style.top = Val::Px(scrolling_list.position);
    }
}

pub struct ScrollableListPlugin;

impl Plugin for ScrollableListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_lists);
    }
}
