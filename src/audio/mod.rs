mod bgm;

use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioPlugin, AudioSource, *};

use crate::GameState;

const MAIN_VOLUME: f64 = 0.35;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_plugins(bgm::BgmPlugin)
            .add_event::<PlaySound>()
            .add_systems(Update, (play_sounds,).run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Event)]
pub struct PlaySound {
    pub clip: Handle<AudioSource>,
    pub volume: f64,
    pub playback_rate: f64,
    pub repeat: bool,
}

impl Default for PlaySound {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            volume: 1.0,
            playback_rate: 1.0,
            repeat: false,
        }
    }
}

fn play_sounds(audio: Res<Audio>, mut ev_play_sound: EventReader<PlaySound>) {
    for ev in ev_play_sound.read() {
        let _audio_instance = if ev.repeat {
            audio
                .play(ev.clip.clone())
                .with_volume(ev.volume * MAIN_VOLUME)
                .with_playback_rate(ev.playback_rate)
                .looped()
                .handle()
        } else {
            audio
                .play(ev.clip.clone())
                .with_volume(ev.volume * MAIN_VOLUME)
                .with_playback_rate(ev.playback_rate)
                .handle()
        };
    }
}
