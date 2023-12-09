use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::PlaySound;

fn play_bgm(assets: Res<GameAssets>, mut ev_play_sound: EventWriter<PlaySound>) {
    ev_play_sound.send(PlaySound {
        clip: assets.bgm.clone(),
        repeat: true,
        ..default()
    });
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), play_bgm);
    }
}
