mod scrollable_list;

use bevy::prelude::*;

use crate::{
    item::{
        item_value::{item_description, item_icon, item_title},
        ActiveItems,
    },
    player::{PlayerChangedState, PlayerState},
    GameAssets, GameState,
};

use scrollable_list::spawn_scrollable_list;

use self::scrollable_list::ScrollingList;

pub struct SpellBookPlugin;

impl Plugin for SpellBookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_spell_book, despawn_spell_bock, update_view).run_if(in_state(GameState::Gaming)),
        )
        .add_plugins(scrollable_list::ScrollableListPlugin);
    }
}

#[derive(Component)]
struct SpellBook;
#[derive(Component)]
struct SpellbookViewIcon;
#[derive(Component)]
struct SpellbookViewTitle;
#[derive(Component)]
struct SpellbookViewDescription;

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) -> Entity {
    commands
        .spawn((ImageBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                texture,
                ..default()
            },
            z_index: ZIndex::Local(-1),
            ..default()
        },))
        .id()
}

fn spawn_scrollable_spell_list(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    active_items: &Res<ActiveItems>,
) -> Entity {
    spawn_scrollable_list(commands, assets, active_items)
}

fn spawn_movement_hint_up(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    commands
        .spawn((ImageBundle {
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
        },))
        .id()
}

fn spawn_movement_hint_down(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    commands
        .spawn((ImageBundle {
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
        },))
        .id()
}

fn spawn_spell_book_view(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let icon = commands
        .spawn((
            SpellbookViewIcon,
            ImageBundle {
                style: Style {
                    top: Val::Percent(16.5),
                    left: Val::Percent(44.87),
                    width: Val::Percent(10.26),
                    height: Val::Percent(16.7),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle {
        text: Text::from_sections([TextSection {
            value: "NOT IMPLEMENTED".to_string(),
            style: text_style,
        }]),
        style: Style {
            top: Val::Percent(45.0),
            left: Val::Percent(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    };
    let title = commands.spawn((SpellbookViewTitle, text_bundle)).id();
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 14.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle {
        text: Text::from_sections([TextSection {
            value: "DESCRIPTION, YOU SHOULD NOT SEE THIS".to_string(),
            style: text_style,
        }]),
        style: Style {
            top: Val::Percent(58.0),
            left: Val::Percent(10.0),
            width: Val::Percent(80.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    };
    let description = commands.spawn((SpellbookViewDescription, text_bundle)).id();

    commands
        .spawn((ImageBundle {
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
        },))
        .push_children(&[icon, title, description])
        .id()
}

fn spawn_spell_book(
    mut commands: Commands,
    assets: Res<GameAssets>,
    active_items: Res<ActiveItems>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::SpellBook {
            continue;
        }

        let background = spawn_background(&mut commands, assets.spell_book_container.clone());
        let scrollable_list = spawn_scrollable_spell_list(&mut commands, &assets, &active_items);
        let hint_up = spawn_movement_hint_up(&mut commands, &assets);
        let hint_down = spawn_movement_hint_down(&mut commands, &assets);
        let view = spawn_spell_book_view(&mut commands, &assets);

        commands
            .spawn((
                SpellBook,
                NodeBundle {
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
            ))
            .push_children(&[background, scrollable_list, hint_up, hint_down, view]);
    }
}

fn despawn_spell_bock(
    mut commands: Commands,
    q_spell_book: Query<Entity, With<SpellBook>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let entity = match q_spell_book.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.old_state == PlayerState::SpellBook {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_view(
    assets: Res<GameAssets>,
    active_items: Res<ActiveItems>,
    q_scrolling_list: Query<&ScrollingList>,
    mut q_view_icon: Query<&mut UiImage, With<SpellbookViewIcon>>,
    mut q_view_title: Query<&mut Text, With<SpellbookViewTitle>>,
    mut q_view_description: Query<
        &mut Text,
        (With<SpellbookViewDescription>, Without<SpellbookViewTitle>),
    >,
) {
    if active_items.is_empty() {
        return;
    }

    let scrolling_list = match q_scrolling_list.get_single() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut icon = match q_view_icon.get_single_mut() {
        Ok(i) => i,
        Err(_) => return,
    };
    let mut title = match q_view_title.get_single_mut() {
        Ok(i) => i,
        Err(_) => return,
    };
    let mut description = match q_view_description.get_single_mut() {
        Ok(i) => i,
        Err(_) => return,
    };

    if scrolling_list.index >= active_items.len() {
        return;
    }
    let item = &active_items[scrolling_list.index];

    let target_texture = item_icon(&assets, item);
    let target_title = item_title(item);
    let target_description = item_description(item);

    if icon.texture != target_texture {
        icon.texture = target_texture;
    }
    title.sections[0].value = target_title;
    description.sections[0].value = target_description;
}
