use bevy::{
    color::palettes::css::RED,
    input::keyboard::{KeyCode, KeyboardInput},
    prelude::*,
    window::PrimaryWindow,
};

use crate::player::{Player, PlayerChangedState, PlayerState};
use crate::{GameAssets, GameState};

const TRANSPARENT_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
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
                "there is not exactly one primary window, not casting spell, {}",
                err
            );
            return;
        }
    };

    let root = commands
        .spawn((
            CastingText,
            Node {
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                width: Val::Px((2.0 * CHAR_SIZE + CHAR_OFFSET) * CHAR_PIXEL_FACTOR),
                height: Val::Px(42.0),
                position_type: PositionType::Absolute,
                right: Val::Px(window.width() / 2.0 - 40.0),
                top: Val::Px(window.height() / 2.0 - 100.0),
                ..default()
            },
            BackgroundColor(TRANSPARENT_BACKGROUND),
        ))
        .id();

    let input_pointer = commands
        .spawn((
            Node {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            Text::from(">"),
            TextFont {
                font: assets.font.clone(),
                font_size: FONT_SIZE_INPUT,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();
    let text = commands
        .spawn((
            TypingBuffer,
            Text::from(""),
            TextFont {
                font: assets.font.clone(),
                font_size: FONT_SIZE_INPUT,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();
    let cursor = commands
        .spawn((
            TypingCursor,
            Text::from("_"),
            TextFont {
                font: assets.font.clone(),
                font_size: FONT_SIZE_INPUT,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    commands
        .entity(root)
        .add_children(&[input_pointer, text, cursor]);
}

fn update_buffer_container(
    typing_state: Res<TypingState>,
    mut q_buffer_container: Query<&mut Node, With<CastingText>>,
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
    q_typing_buffer_text: Query<Entity, With<TypingBuffer>>,
    mut writer: TextUiWriter,
) {
    if !typing_state.is_changed() {
        return;
    }

    let text = match q_typing_buffer_text.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };
    *writer.text(text, 0) = typing_state.buf.clone();
}

fn update_cursor_text(
    mut timer: ResMut<TypingCursorTimer>,
    mut query: Query<&mut TextColor, With<TypingCursor>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for mut target in query.iter_mut() {
        if target.0 != Color::NONE {
            target.0 = Color::NONE;
        } else {
            target.0 = RED.into();
        }
    }
}

fn push_chars(
    mut typing_state: ResMut<TypingState>,
    mut typing_submit_events: EventWriter<TypingSubmitEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    q_player: Query<&Player>,
) {
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };

    let control_active = keys.pressed(KeyCode::ControlLeft);

    for ev in keyboard_input_events.read() {
        // We run this in the loop so that the events get consumed.
        // Otherwise we might run into the issue of added it to the buffer
        // when spawning the text field.
        if player_state != PlayerState::Casting {
            continue;
        }

        if ev.state.is_pressed() {
            let maybe_char = match ev.key_code {
                KeyCode::KeyA => Some('a'),
                KeyCode::KeyB => Some('b'),
                KeyCode::KeyC => Some('c'),
                KeyCode::KeyD => Some('d'),
                KeyCode::KeyE => Some('e'),
                KeyCode::KeyF => Some('f'),
                KeyCode::KeyG => Some('g'),
                KeyCode::KeyH => Some('h'),
                KeyCode::KeyI => Some('i'),
                KeyCode::KeyJ => Some('j'),
                KeyCode::KeyK => Some('k'),
                KeyCode::KeyL => Some('l'),
                KeyCode::KeyM => Some('m'),
                KeyCode::KeyN => Some('n'),
                KeyCode::KeyO => Some('o'),
                KeyCode::KeyP => Some('p'),
                KeyCode::KeyQ => Some('q'),
                KeyCode::KeyR => Some('r'),
                KeyCode::KeyS => Some('s'),
                KeyCode::KeyT => Some('t'),
                KeyCode::KeyU => Some('u'),
                KeyCode::KeyV => Some('v'),
                KeyCode::KeyW => {
                    if !control_active {
                        Some('w')
                    } else {
                        typing_state.buf = trim_last_word(&typing_state.buf);
                        None
                    }
                }
                KeyCode::KeyX => Some('x'),
                KeyCode::KeyY => Some('y'),
                KeyCode::KeyZ => Some('z'),
                KeyCode::Space => Some(' '),
                KeyCode::Minus => Some('-'),
                _ => None,
            };

            if let Some(char) = maybe_char {
                typing_state.buf.push(char);
                typing_state.just_typed_char = true;
            } else {
                typing_state.just_typed_char = false;
            }

            if ev.key_code == KeyCode::Enter {
                let text = typing_state.buf.clone();
                typing_submit_events.send(TypingSubmitEvent { value: text });
            }

            if ev.key_code == KeyCode::Backspace {
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
