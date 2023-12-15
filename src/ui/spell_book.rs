use bevy::prelude::*;

use crate::{player::input::PlayerInput, GameAssets, GameState};

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

fn spawn_scrollable_spell_list(commands: &mut Commands) -> Entity {
    spawn_scrollable_list(commands)
}

fn spawn_spell_book(commands: &mut Commands, assets: &Res<GameAssets>) {
    let background = spawn_background(commands, assets.spell_book_container.clone());
    let scrollable_list = spawn_scrollable_spell_list(commands);

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
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[background, scrollable_list]);
}

fn despawn_spell_bock(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn_recursive();
}

fn toggle_spell_bock(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    q_spell_book: Query<Entity, With<SpellBook>>,
) {
    if !player_input.toggle_spell_book {
        return;
    }

    match q_spell_book.get_single() {
        Ok(s) => despawn_spell_bock(&mut commands, s),
        Err(_) => spawn_spell_book(&mut commands, &assets),
    };
}

pub struct SpellBookPlugin;

impl Plugin for SpellBookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_spell_bock,).run_if(in_state(GameState::Gaming)),
        );
    }
}
