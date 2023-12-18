use bevy::prelude::*;

use super::{Spell, SpellCasted};

#[derive(Resource, Default)]
pub struct DebugSpell {
    pub active: bool,
}

fn toggle_debug_mod(
    mut debug_spell: ResMut<DebugSpell>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Debug {
            debug_spell.active = !debug_spell.active;
        }
    }
}

pub struct DebugSpellPlugin;

impl Plugin for DebugSpellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugSpell>()
            .add_systems(Update, toggle_debug_mod);
    }
}
