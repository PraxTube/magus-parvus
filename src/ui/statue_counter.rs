use bevy::prelude::*;

use crate::{
    item::{statue::StatueUnlockedDelayed, ActiveItems, MaxItems},
    GameAssets, GameState,
};

#[derive(Component)]
struct TextCounter;

fn spawn_statue_counter(mut commands: Commands, assets: Res<GameAssets>) {
    let image = commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: assets.statue_ui_icon.clone(),
                ..default()
            },
            ..default()
        })
        .id();
    let text = commands
        .spawn((
            TextCounter,
            TextBundle {
                style: Style {
                    top: Val::Px(24.0),
                    ..default()
                },
                text: Text::from_sections([TextSection {
                    value: String::new(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }]),
                ..default()
            },
        ))
        .id();

    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(10.0),
                right: Val::Px(25.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .push_children(&[image, text]);
}

fn update_text_counter(
    active_items: Res<ActiveItems>,
    max_items: Res<MaxItems>,
    mut q_text_counter: Query<&mut Text, With<TextCounter>>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    let mut text = match q_text_counter.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    text.sections[0].value = format!(" {}/{}", active_items.len(), max_items.0);
}

pub struct StatueCounterUiPlugin;

impl Plugin for StatueCounterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_statue_counter,))
            .add_systems(
                Update,
                (update_text_counter,).run_if(resource_exists::<MaxItems>()),
            );
    }
}
