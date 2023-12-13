use bevy::prelude::*;

use crate::{player::Player, ui::health::Health, GameState};

use super::{Spell, SpellCasted};

fn kill_player(
    mut q_player: Query<&mut Health, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let mut player_health = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::KillPlayer {
            player_health.health = 0.0;
        }
    }
}

pub struct KillPlayerPlugin;

impl Plugin for KillPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (kill_player,).run_if(in_state(GameState::Gaming)));
    }
}
