use bevy::prelude::*;

use crate::item::statue::StatueUnlockedDelayed;
use crate::item::Item;
use crate::{GameAssets, GameState};

#[derive(Component)]
struct PopUp {
    timer: Timer,
}

impl PopUp {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(2.5, TimerMode::Once),
        }
    }
}

fn spawn_title_text(
    commands: &mut Commands,
    font: Handle<Font>,
    ev: &StatueUnlockedDelayed,
) -> Entity {
    let text = match ev.statue.item {
        Item::Test => "TEST, you should not see this, please report",
        Item::Fulgur => "UNLOCKED: Fulgur",
    };

    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle {
        text: Text::from_sections([TextSection::new(text, text_style)]),
        style: Style {
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };
    commands.spawn(text_bundle).id()
}

fn spawn_pop_up(commands: &mut Commands, font: Handle<Font>, ev: &StatueUnlockedDelayed) {
    let title_text = spawn_title_text(commands, font.clone(), ev);

    commands
        .spawn((
            PopUp::new(),
            NodeBundle {
                style: Style {
                    top: Val::Percent(20.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(10.0),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[title_text]);
}

fn spawn_pop_ups(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_unlocked: EventReader<StatueUnlockedDelayed>,
) {
    for ev in ev_statue_unlocked.read() {
        spawn_pop_up(&mut commands, assets.font.clone(), ev);
    }
}

fn despawn_pop_ups(
    mut commands: Commands,
    time: Res<Time>,
    mut pop_ups: Query<(Entity, &mut PopUp)>,
) {
    for (entity, mut pop_up) in &mut pop_ups {
        pop_up.timer.tick(time.delta());

        if pop_up.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct PopUpPlugin;

impl Plugin for PopUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_pop_ups, despawn_pop_ups).run_if(in_state(GameState::Gaming)),
        );
    }
}
