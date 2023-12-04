pub mod fireball;
pub mod icicle;
pub mod lightning;
mod phantasma;
mod speed_boost;

use std::time::Duration;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;

use crate::player::{Player, PlayerState};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (write_spell, double_j_escape, submit_spell))
            .add_event::<SpellCasted>()
            .add_plugins((
                fireball::FireballPlugin,
                lightning::LightningPlugin,
                icicle::IciclePlugin,
                speed_boost::SpeedBoostPlugin,
                phantasma::PhantasmaPlugin,
            ));
    }
}

#[derive(PartialEq)]
enum Spell {
    Fireball,
    IgnisPila,
    InfernoPila,
    Fulgur,
    ScutumGlaciei,
    SpeedBoost,
    Phantasma,
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
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    if player_state != PlayerState::Casting {
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
    let mut player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

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
    let mut player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_input_submitted.read() {
        player.state = PlayerState::Idling;

        if ev.value.to_lowercase() == "fireball" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Fireball,
            });
        } else if ev.value.to_lowercase() == "ignis pila" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::IgnisPila,
            });
        } else if ev.value.to_lowercase() == "inferno pila" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::InfernoPila,
            });
        } else if ev.value.to_lowercase() == "fulgur" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Fulgur,
            });
        } else if ev.value.to_lowercase() == "scutum glaciei" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::ScutumGlaciei,
            });
        } else if ev.value.to_lowercase() == "cito" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::SpeedBoost,
            });
        } else if ev.value.to_lowercase() == "phantasma" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Phantasma,
            });
        }
    }
}
