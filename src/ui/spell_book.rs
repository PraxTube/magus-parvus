use bevy::prelude::*;

use crate::{
    item::Item,
    player::{PlayerChangedState, PlayerState},
    GameAssets, GameState,
};

use super::scrollable_list::spawn_scrollable_list;

#[derive(Component)]
struct SpellBook;

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

fn spawn_scrollable_spell_list(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    spawn_scrollable_list(
        commands,
        assets,
        vec![
            Item::Fulgur,
            Item::InfernoPila,
            Item::ScutumGlaciei,
            Item::Fulgur,
            Item::InfernoPila,
            Item::Fulgur,
            Item::InfernoPila,
            Item::ScutumGlaciei,
        ],
    )
}

fn spawn_spell_book(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::SpellBook {
            continue;
        }

        let background = spawn_background(&mut commands, assets.spell_book_container.clone());
        let scrollable_list = spawn_scrollable_spell_list(&mut commands, &assets);

        commands
            .spawn((
                SpellBook,
                NodeBundle {
                    style: Style {
                        height: Val::Percent(80.0),
                        width: Val::Percent(40.0),
                        top: Val::Percent(10.0),
                        left: Val::Percent(30.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    z_index: ZIndex::Local(200),
                    ..default()
                },
            ))
            .push_children(&[background, scrollable_list]);
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

pub struct SpellBookPlugin;

impl Plugin for SpellBookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_spell_book, despawn_spell_bock).run_if(in_state(GameState::Gaming)),
        );
    }
}
