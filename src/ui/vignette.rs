use bevy::prelude::*;

use crate::{item::platform::TriggerFinalAct, GameAssets, GameState};

fn spawn_vignette(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage {
            texture: assets.vignette.clone(),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });
}

pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_vignette,).run_if(in_state(GameState::Gaming)),
        );
    }
}
