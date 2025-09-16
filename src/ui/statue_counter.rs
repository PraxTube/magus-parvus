use bevy::prelude::*;

use crate::{
    item::{statue::StatueUnlockedDelayed, ActiveItems, MaxItems},
    GameAssets, GameState,
};

#[derive(Component)]
struct TextCounter;

fn spawn_statue_counter(mut commands: Commands, assets: Res<GameAssets>) {
    let image = commands
        .spawn((
            ImageNode {
                image: assets.statue_ui_icon.clone(),
                ..default()
            },
            Node {
                position_type: PositionType::Relative,
                width: Val::Px(48.0),
                height: Val::Px(80.0),
                ..default()
            },
        ))
        .id();
    let text = commands
        .spawn((
            TextCounter,
            Node {
                top: Val::Px(24.0),
                ..default()
            },
            Text::from(String::new()),
            TextFont {
                font: assets.font.clone(),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    commands
        .spawn(Node {
            top: Val::Px(10.0),
            right: Val::Px(25.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .add_children(&[image, text]);
}

fn update_text_counter(
    active_items: Res<ActiveItems>,
    max_items: Res<MaxItems>,
    mut q_text_counter: Query<Entity, With<TextCounter>>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
    mut writer: TextUiWriter,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    let text = match q_text_counter.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    *writer.text(text, 0) = format!(" {}/{}", active_items.len(), max_items.0);
}

pub struct StatueCounterUiPlugin;

impl Plugin for StatueCounterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_statue_counter,))
            .add_systems(
                Update,
                (update_text_counter,).run_if(resource_exists::<MaxItems>),
            );
    }
}
