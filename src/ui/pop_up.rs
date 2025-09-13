use bevy::prelude::*;

use crate::item::item_value::{item_description, item_title};
use crate::item::statue::StatueUnlockedDelayed;
use crate::{GameAssets, GameState};

const TIME: f32 = 5.0;

#[derive(Component)]
struct PopUp {
    timer: Timer,
}

impl PopUp {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(TIME, TimerMode::Once),
        }
    }
}

fn spawn_item_title(commands: &mut Commands, font: Handle<Font>, text: String) -> Entity {
    let text_font = TextFont {
        font,
        font_size: 40.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);
    let text = Text::from(text);
    commands.spawn((text, text_font, text_color)).id()
}

fn spawn_item_description(commands: &mut Commands, font: Handle<Font>, text: String) -> Entity {
    let text_font = TextFont {
        font,
        font_size: 24.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);
    let text = Text::from(text);
    let layout = TextLayout {
        justify: JustifyText::Center,
        ..default()
    };
    commands.spawn((text, text_font, text_color, layout)).id()
}

fn spawn_pop_up(commands: &mut Commands, font: Handle<Font>, ev: &StatueUnlockedDelayed) {
    let title = spawn_item_title(commands, font.clone(), item_title(&ev.statue.item));
    let description = spawn_item_description(commands, font, item_description(&ev.statue.item));

    commands
        .spawn((
            PopUp::new(),
            Node {
                top: Val::Percent(20.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Vh(10.0),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ZIndex(101),
        ))
        .add_children(&[title, description]);
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
