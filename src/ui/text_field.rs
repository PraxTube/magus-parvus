use bevy::{
    input::{keyboard::KeyCode, keyboard::KeyboardInput},
    prelude::*,
    window::PrimaryWindow,
};

use crate::player::{Player, PlayerChangedState, PlayerState};
use crate::{GameAssets, GameState};

const TRANSPARENT_BACKGROUND: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);
const FONT_SIZE_INPUT: f32 = 32.0;
const CHAR_SIZE: f32 = 2.5;
const CHAR_OFFSET: f32 = 1.5;
const CHAR_PIXEL_FACTOR: f32 = 12.8;

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

fn trim_last_word(s: &str) -> String {
    let trimmed_str = s.trim_end();
    match trimmed_str.rfind(' ') {
        Some(space_index) => trimmed_str[..space_index + 1].to_string(),
        None => String::new(),
    }
}

fn spawn_text_field(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    q_window: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = match q_window.get_single() {
        Ok(w) => w,
        Err(err) => {
            error!(
                "there is not exactly one primary window, not casting spel, {}",
                err
            );
            return;
        }
    };

    let root = commands
        .spawn((
            CastingText,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    width: Val::Px((2.0 * CHAR_SIZE + CHAR_OFFSET) * CHAR_PIXEL_FACTOR),
                    height: Val::Px(42.0),
                    position_type: PositionType::Absolute,
                    right: Val::Px(window.width() / 2.0 - 40.0),
                    top: Val::Px(window.height() / 2.0 - 100.0),
                    ..default()
                },
                background_color: TRANSPARENT_BACKGROUND.into(),
                ..default()
            },
        ))
        .id();

    let input_pointer = commands
        .spawn(TextBundle {
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
        })
        .id();
    let text = commands
        .spawn((
            TypingBuffer,
            TextBundle {
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
        ))
        .id();
    let cursor = commands
        .spawn((
            TypingCursor,
            TextBundle {
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
        ))
        .id();

    commands
        .entity(root)
        .push_children(&[input_pointer, text, cursor]);
}

fn update_buffer_container(
    typing_state: Res<TypingState>,
    mut q_buffer_container: Query<&mut Style, With<CastingText>>,
) {
    if !typing_state.is_changed() {
        return;
    }

    let mut style = match q_buffer_container.get_single_mut() {
        Ok(s) => s,
        Err(_) => return,
    };

    let k = 2.0 + typing_state.buf.len() as f32;
    style.width = Val::Px((k * CHAR_SIZE + CHAR_OFFSET) * CHAR_PIXEL_FACTOR);
}

fn update_buffer_text(
    typing_state: Res<TypingState>,
    mut q_typing_buffer_text: Query<&mut Text, With<TypingBuffer>>,
) {
    if !typing_state.is_changed() {
        return;
    }

    let mut text = match q_typing_buffer_text.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    text.sections[0].value.clone_from(&typing_state.buf);
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

fn push_chars(
    mut typing_state: ResMut<TypingState>,
    mut typing_submit_events: EventWriter<TypingSubmitEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    keys: Res<Input<KeyCode>>,
    q_player: Query<&Player>,
) {
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    if player_state != PlayerState::Casting {
        return;
    }

    let control_active = keys.pressed(KeyCode::ControlLeft);

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
                Some(KeyCode::W) => {
                    if !control_active {
                        Some('w')
                    } else {
                        typing_state.buf = trim_last_word(&typing_state.buf);
                        None
                    }
                }
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
                typing_submit_events.send(TypingSubmitEvent { value: text });
            }

            if ev.key_code == Some(KeyCode::Back) {
                if !control_active {
                    typing_state.buf.pop();
                } else {
                    typing_state.buf = trim_last_word(&typing_state.buf);
                }
            }
        }
    }
}

fn spawn_casting_text(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut typing_state: ResMut<TypingState>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.new_state == PlayerState::Casting {
            typing_state.buf.clear();
            spawn_text_field(&mut commands, &assets, &q_window);
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
                push_chars,
                update_cursor_text,
                update_buffer_container.after(push_chars),
                update_buffer_text.after(push_chars),
                despawn_casting_text,
                spawn_casting_text.run_if(in_state(GameState::Gaming)),
            ),
        );
    }
}
