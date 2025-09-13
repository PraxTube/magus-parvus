mod bundle;
mod scrollable_list;

use crate::{
    item::{
        item_value::{item_description, item_icon, item_title},
        ActiveItems,
    },
    player::{PlayerChangedState, PlayerState},
    GameAssets, GameState,
};
use bevy::prelude::*;

use bundle::{
    BackgroundBundle, MovementHintDownBundle, MovementHintUpBundle, SpellbookBundle,
    SpellbookViewBundle, SpellbookViewDescriptionBundle, SpellbookViewIconBundle,
    SpellbookViewTitleBundle,
};
use scrollable_list::{spawn_scrollable_list, ScrollingList};

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

fn spawn_spell_book_view(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let icon = commands.spawn(SpellbookViewIconBundle::default()).id();
    let title = commands.spawn(SpellbookViewTitleBundle::new(assets)).id();
    let description = commands
        .spawn(SpellbookViewDescriptionBundle::new(assets))
        .id();

    commands
        .spawn(SpellbookViewBundle::new(assets))
        .add_children(&[icon, title, description])
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

        let background = commands.spawn(BackgroundBundle::new(&assets)).id();
        let hint_up = commands.spawn(MovementHintUpBundle::new(&assets)).id();
        let hint_down = commands.spawn(MovementHintDownBundle::new(&assets)).id();
        let scrollable_list = spawn_scrollable_list(&mut commands, &assets, &active_items);
        let view = spawn_spell_book_view(&mut commands, &assets);

        commands.spawn(SpellbookBundle::default()).add_children(&[
            background,
            scrollable_list,
            hint_up,
            hint_down,
            view,
        ]);
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
    mut q_view_icon: Query<&mut ImageNode, With<SpellbookViewIcon>>,
    q_view_title: Query<Entity, With<SpellbookViewTitle>>,
    q_view_description: Query<
        Entity,
        (With<SpellbookViewDescription>, Without<SpellbookViewTitle>),
    >,
    mut writer: TextUiWriter,
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
    let title = match q_view_title.get_single() {
        Ok(i) => i,
        Err(_) => return,
    };
    let description = match q_view_description.get_single() {
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

    if icon.image != target_texture {
        icon.image = target_texture;
    }

    *writer.text(title, 0) = target_title;
    *writer.text(description, 0) = target_description;
}
