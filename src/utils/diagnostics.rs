use bevy::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin,
};

use crate::{spell::debug_spell::DebugSpell, GameState};

#[derive(Resource, Default)]
struct Diagnostics {
    active: bool,
}

fn toggle(screen_diags: &mut ResMut<ScreenDiagnostics>) {
    screen_diags.modify("fps").toggle();
    screen_diags.modify("ms/frame").toggle();
    screen_diags.modify("entities").toggle();
}

fn toggle_off(mut screen_diags: ResMut<ScreenDiagnostics>) {
    toggle(&mut screen_diags);
}

fn toggle_diagnostics(
    debug_spell: Res<DebugSpell>,
    mut diags: ResMut<Diagnostics>,
    mut screen_diags: ResMut<ScreenDiagnostics>,
) {
    if debug_spell.active != diags.active {
        diags.active = debug_spell.active;
        toggle(&mut screen_diags);
    }
}

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreenDiagnosticsPlugin {
                timestep: 1.0,
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        .init_resource::<Diagnostics>()
        .add_systems(OnEnter(GameState::Gaming), toggle_off)
        .add_systems(
            Update,
            toggle_diagnostics.run_if(resource_changed::<DebugSpell>()),
        );
    }
}
