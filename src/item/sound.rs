use bevy::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::statue::StatueUnlockedDelayed;

fn spawn_item_unlock_sounds(
    assets: Res<GameAssets>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for _ in ev_statue_unlocked_delayed.read() {
        ev_play_sound.send(PlaySound {
            clip: assets.item_unlock_sound.clone(),
            ..default()
        });
    }
}

pub struct ItemSoundPlugin;

impl Plugin for ItemSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_item_unlock_sounds,).run_if(in_state(GameState::Gaming)),
        );
    }
}
