use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitEvent};

use crate::{GameState, Player, PlayerChangedState, PlayerState};

pub const BORANGE: Color = Color::rgb(
    0xDF as f32 / 255.0,
    0x71 as f32 / 255.0,
    0x26 as f32 / 255.0,
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin).add_systems(
            Update,
            (cast_spell, spawn_casting_text, despawn_casting_text)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Component)]
pub struct CastingText;

fn spawn_text_field(commands: &mut Commands) {
    let root = commands
        .spawn((
            CastingText,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let text_input_field = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(35.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: BORANGE.into(),
                background_color: Color::BLACK.into(),
                ..default()
            },
            TextInput {
                text_style: TextStyle {
                    font_size: 40.,
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text_input_field]);
}

fn spawn_casting_text(
    mut commands: Commands,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.new_state == PlayerState::Casting {
            spawn_text_field(&mut commands);
        }
    }
}

fn despawn_casting_text(
    mut commands: Commands,
    q_casting_text: Query<Entity, With<CastingText>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.old_state == PlayerState::Casting {
            let entity = q_casting_text.single();
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cast_spell(
    mut q_player: Query<&mut Player>,
    mut ev_input_submitted: EventReader<TextInputSubmitEvent>,
) {
    for event in ev_input_submitted.read() {
        let mut player = q_player.single_mut();
        player.state = PlayerState::Idling;
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}
