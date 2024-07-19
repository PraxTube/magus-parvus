use std::time::Duration;

use bevy::prelude::*;

use crate::{
    item::{item_value::item_spell, ActiveItems},
    player::{Player, PlayerState},
    ui::text_field::TypingSubmitEvent,
};

use super::{debug_spell::DebugSpell, Spell, SpellCasted};

fn double_j_escape(
    keys: Res<ButtonInput<KeyCode>>,
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
    if keys.just_pressed(KeyCode::KeyJ) {
        if timer.finished() {
            timer.set_duration(duration);
            timer.reset();
            return;
        }

        player.state = PlayerState::Idling;
    }
}

fn is_spell_active(active_items: &Res<ActiveItems>, spell: &Spell) -> bool {
    if spell == &Spell::Debug || spell == &Spell::Flub {
        return true;
    }

    for item in &active_items.0 {
        if &item_spell(item) == spell {
            return true;
        }
    }
    false
}

fn submit_spell(
    active_items: Res<ActiveItems>,
    debug_spell: Res<DebugSpell>,
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
        if let Ok(spell) = &ev.value.parse::<Spell>() {
            if debug_spell.active || is_spell_active(&active_items, spell) {
                ev_spell_casted.send(SpellCasted {
                    spell: spell.clone(),
                });
            }
        }
    }
}

pub struct CastSpellPlugin;

impl Plugin for CastSpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (double_j_escape, submit_spell));
    }
}
