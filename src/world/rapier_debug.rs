use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::spell::debug_spell::DebugSpell;

fn toggle_rapier_debug(
    mut debug_context: ResMut<DebugRenderContext>,
    debug_spell: Res<DebugSpell>,
) {
    if debug_context.enabled != debug_spell.active {
        debug_context.enabled = debug_spell.active;
    }
}

pub struct RapierDebugPlugin;

impl Plugin for RapierDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_rapier_debug,));
    }
}
