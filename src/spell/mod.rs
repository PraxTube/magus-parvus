pub mod aer_tracto;
pub mod fireball;
pub mod icicle;
pub mod lightning;
pub mod lightning_bird;

mod death;
mod flub;
mod kill_player;
mod phantasma;
mod speed_boost;

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    player::{Player, PlayerState},
    ui::text_field::TypingSubmitEvent,
};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fireball::FireballPlugin,
            lightning::LightningPlugin,
            lightning_bird::LightningBirdPlugin,
            icicle::IciclePlugin,
            aer_tracto::AerTractoPlugin,
            speed_boost::SpeedBoostPlugin,
            phantasma::PhantasmaPlugin,
            death::DeathPlugin,
            flub::FlubPlugin,
            kill_player::KillPlayerPlugin,
        ))
        .add_event::<SpellCasted>()
        .add_systems(Update, (double_j_escape, submit_spell));
    }
}

#[derive(PartialEq)]
enum Spell {
    Fireball,
    IgnisPila,
    InfernoPila,
    Fulgur,
    FulgurAvis,
    ScutumGlaciei,
    AerTracto,
    AerPello,
    SpeedBoost,
    Phantasma,
    Death,
    Flub,
    KillPlayer,
}

#[derive(Event)]
pub struct SpellCasted {
    spell: Spell,
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
    mut ev_typing_submit_event: EventReader<TypingSubmitEvent>,
    mut ev_spell_casted: EventWriter<SpellCasted>,
) {
    let mut player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_typing_submit_event.read() {
        player.state = PlayerState::Idling;
        let spell_str = ev.value.trim_start().trim_end().to_lowercase();

        if spell_str == "fireball" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Fireball,
            });
        } else if spell_str == "ignis pila" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::IgnisPila,
            });
        } else if spell_str == "inferno pila" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::InfernoPila,
            });
        } else if spell_str == "fulgur" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Fulgur,
            });
        } else if spell_str == "fulgur avis" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::FulgurAvis,
            });
        } else if spell_str == "scutum glaciei" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::ScutumGlaciei,
            });
        } else if spell_str == "aer tracto" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::AerTracto,
            });
        } else if spell_str == "aer pello" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::AerPello,
            });
        } else if spell_str == "cito" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::SpeedBoost,
            });
        } else if spell_str == "phantasma" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Phantasma,
            });
        } else if spell_str == "now you" || spell_str == "jetzt du" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::Death,
            });
        } else if spell_str == "kill player" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::KillPlayer,
            });
        } else if !spell_str.is_empty() {
            ev_spell_casted.send(SpellCasted { spell: Spell::Flub })
        }
    }
}
