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
mod spell;

use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use bevy::prelude::*;

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
            spell::SpellPlugin,
        ))
        .add_event::<SpellCasted>();
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug)]
struct InvalidSpell;

impl Display for InvalidSpell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not a valid spell")
    }
}

impl Error for InvalidSpell {}

impl FromStr for Spell {
    type Err = InvalidSpell;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spell_str: &str = &s.trim_start().trim_end().to_lowercase();
        if spell_str.is_empty() {
            return Err(InvalidSpell);
        }

        match spell_str {
            "fireball" => Ok(Spell::Fireball),
            "ignis pila" => Ok(Spell::IgnisPila),
            "inferno pila" => Ok(Spell::InfernoPila),
            "fulgur" => Ok(Spell::Fulgur),
            "fulgur avis" => Ok(Spell::FulgurAvis),
            "scutum glaciei" => Ok(Spell::ScutumGlaciei),
            "aer tracto" => Ok(Spell::AerTracto),
            "aer pello" => Ok(Spell::AerPello),
            "cito" => Ok(Spell::SpeedBoost),
            "phantasma" => Ok(Spell::Phantasma),
            "now you" | "jetzt du" => Ok(Spell::Death),
            "kill player" => Ok(Spell::KillPlayer),
            _ => Ok(Spell::Flub),
        }
    }
}

#[derive(Event)]
pub struct SpellCasted {
    spell: Spell,
}
