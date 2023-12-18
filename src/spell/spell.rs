use std::time::Duration;

use bevy::prelude::*;

use crate::{
    player::{Player, PlayerState},
    ui::text_field::TypingSubmitEvent,
};

use super::{Spell, SpellCasted};

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
        if let Ok(spell) = &ev.value.parse::<Spell>() {
            ev_spell_casted.send(SpellCasted {
                spell: spell.clone(),
            });
        }
    }
}

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (double_j_escape, submit_spell));
    }
}
