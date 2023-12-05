use bevy::{
    input::{keyboard::KeyCode, keyboard::KeyboardInput},
    prelude::*,
};

use crate::{
    player::{Player, PlayerChangedState, PlayerState},
    GameAssets, GameState,
};

const TRANSPARENT_BACKGROUND: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);
const FONT_SIZE_INPUT: f32 = 32.0;

#[derive(Component)]
pub struct CastingText;
#[derive(Component)]
struct TypingBuffer;
#[derive(Component)]
struct TypingCursor;
#[derive(Resource)]
struct TypingCursorTimer(Timer);

#[derive(Event)]
pub struct TypingSubmitEvent {
    pub value: String,
}

#[derive(Resource, Default, Debug)]
pub struct TypingState {
    buf: String,
    just_typed_char: bool,
}

fn spawn_text_field(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            CastingText,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(42.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..default()
                },
                background_color: TRANSPARENT_BACKGROUND.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    ">".to_string(),
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: FONT_SIZE_INPUT,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "".to_string(),
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: FONT_SIZE_INPUT,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                TypingBuffer,
            ));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "_".to_string(),
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: FONT_SIZE_INPUT,
                            color: Color::RED,
                        },
                    ),
                    ..default()
                },
                TypingCursor,
            ));
        });
}

fn update_buffer_text(state: Res<TypingState>, mut query: Query<&mut Text, With<TypingBuffer>>) {
    if !state.is_changed() {
        return;
    }

    for mut target in query.iter_mut() {
        target.sections[0].value.clone_from(&state.buf);
    }
}

fn update_cursor_text(
    mut timer: ResMut<TypingCursorTimer>,
    mut query: Query<&mut Text, With<TypingCursor>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for mut target in query.iter_mut() {
        if target.sections[0].style.color != Color::NONE {
            target.sections[0].style.color = Color::NONE;
        } else {
            target.sections[0].style.color = Color::RED;
        }
    }
}

fn keyboard(
    mut typing_state: ResMut<TypingState>,
    mut typing_submit_events: EventWriter<TypingSubmitEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    q_player: Query<&Player>,
) {
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    if player_state != PlayerState::Casting {
        return;
    }

    for ev in keyboard_input_events.read() {
        if ev.state.is_pressed() {
            let maybe_char = match ev.key_code {
                Some(KeyCode::A) => Some('a'),
                Some(KeyCode::B) => Some('b'),
                Some(KeyCode::C) => Some('c'),
                Some(KeyCode::D) => Some('d'),
                Some(KeyCode::E) => Some('e'),
                Some(KeyCode::F) => Some('f'),
                Some(KeyCode::G) => Some('g'),
                Some(KeyCode::H) => Some('h'),
                Some(KeyCode::I) => Some('i'),
                Some(KeyCode::J) => Some('j'),
                Some(KeyCode::K) => Some('k'),
                Some(KeyCode::L) => Some('l'),
                Some(KeyCode::M) => Some('m'),
                Some(KeyCode::N) => Some('n'),
                Some(KeyCode::O) => Some('o'),
                Some(KeyCode::P) => Some('p'),
                Some(KeyCode::Q) => Some('q'),
                Some(KeyCode::R) => Some('r'),
                Some(KeyCode::S) => Some('s'),
                Some(KeyCode::T) => Some('t'),
                Some(KeyCode::U) => Some('u'),
                Some(KeyCode::V) => Some('v'),
                Some(KeyCode::W) => Some('w'),
                Some(KeyCode::X) => Some('x'),
                Some(KeyCode::Y) => Some('y'),
                Some(KeyCode::Z) => Some('z'),
                Some(KeyCode::Space) => Some(' '),
                Some(KeyCode::Minus) => Some('-'),
                _ => None,
            };

            if let Some(char) = maybe_char {
                typing_state.buf.push(char);
                typing_state.just_typed_char = true;
            } else {
                typing_state.just_typed_char = false;
            }

            if ev.key_code == Some(KeyCode::Return) {
                let text = typing_state.buf.clone();

                typing_state.buf.clear();
                typing_submit_events.send(TypingSubmitEvent { value: text });
            }

            if ev.key_code == Some(KeyCode::Back) {
                typing_state.buf.pop();
            }

            if ev.key_code == Some(KeyCode::Escape) {
                typing_state.buf.clear();
            }
        }
    }
}

fn spawn_casting_text(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut typing_state: ResMut<TypingState>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.new_state == PlayerState::Casting {
            typing_state.buf.clear();
            spawn_text_field(&mut commands, &assets);
        }
    }
}

fn despawn_casting_text(
    mut commands: Commands,
    q_casting_text: Query<Entity, With<CastingText>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let entity = match q_casting_text.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.old_state == PlayerState::Casting {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TypingCursorTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .init_resource::<TypingState>()
        .add_event::<TypingSubmitEvent>()
        .add_systems(
            Update,
            (
                update_cursor_text,
                keyboard,
                update_buffer_text.after(keyboard),
                spawn_casting_text,
                despawn_casting_text,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
