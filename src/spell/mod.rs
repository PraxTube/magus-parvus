mod fireball;

use std::time::Duration;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;

use crate::{Player, PlayerState};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (write_spell, double_j_escape, submit_spell))
            .add_event::<SpellCasted>()
            .add_plugins(fireball::FireballPlugin);
    }
}

#[derive(PartialEq)]
enum Spell {
    Fireball,
    FireballCircle,
}

#[derive(Event)]
pub struct SpellCasted {
    spell: Spell,
}

fn write_spell(
    keys: Res<Input<KeyCode>>,
    q_player: Query<&Player>,
    mut string: Local<String>,
    mut ev_received_char: EventReader<ReceivedCharacter>,
) {
    let player = q_player.single();
    if player.state != PlayerState::Casting {
        return;
    }

    if keys.just_pressed(KeyCode::Return) {
        string.clear();
    }
    if keys.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in ev_received_char.read() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
}

fn double_j_escape(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_player: Query<&mut Player>,
    mut timer: Local<Timer>,
) {
    let mut player = q_player.single_mut();
    let duration = Duration::from_secs_f32(0.2);

    if player.state != PlayerState::Casting {
        timer.set_elapsed(duration);
        return;
    }

    timer.tick(time.delta());
    if keys.just_pressed(KeyCode::J) {
        if timer.finished() {
            timer.set_duration(duration);
            timer.reset();
            return;
        }

        player.state = PlayerState::Idling;
    }
}

fn submit_spell(
    mut q_player: Query<&mut Player>,
    mut ev_input_submitted: EventReader<TextInputSubmitEvent>,
    mut ev_spell_casted: EventWriter<SpellCasted>,
) {
    for ev in ev_input_submitted.read() {
        let mut player = q_player.single_mut();
        player.state = PlayerState::Idling;

        if ev.value.to_lowercase() == "fireball" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Fireball,
            });
        } else if ev.value.to_lowercase() == "ignis pila" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::FireballCircle,
            });
        }
    }
}
