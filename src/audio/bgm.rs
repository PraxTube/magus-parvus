use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{GameAssets, GameState};

use super::GameAudio;

const BGM_VOLUME: f64 = 0.5;

#[derive(Component)]
struct Bgm {
    handle: Handle<AudioInstance>,
}

fn play_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
) {
    let volume = game_audio.main_volume * BGM_VOLUME;
    let handle = audio
        .play(assets.bgm.clone())
        .with_volume(volume)
        .looped()
        .handle();
    commands.spawn(Bgm { handle });
}

fn update_bgm_volumes(
    game_audio: Res<GameAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<&Bgm>,
) {
    let volume = game_audio.main_volume * BGM_VOLUME;
    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            instance.set_volume(volume, AudioTween::default());
        }
    }
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), play_bgm)
            .add_systems(
                Update,
                (update_bgm_volumes,).run_if(in_state(GameState::GameOver)),
            );
    }
}
