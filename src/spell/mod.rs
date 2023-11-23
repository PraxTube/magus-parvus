mod fireball;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;

use crate::{Player, PlayerState};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cast_spell,))
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

fn cast_spell(
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
        } else if ev.value.to_lowercase() == "fireballs" {
            ev_spell_casted.send(SpellCasted {
                spell: Spell::FireballCircle,
            });
        }
    }
}
