mod bgm;
mod spacial;

use rand::{thread_rng, Rng};

use bevy::{prelude::*, utils::HashSet};
use bevy_kira_audio::prelude::{AudioPlugin, AudioSource, *};

use crate::GameState;

const MAIN_VOLUME: f64 = 0.5;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_plugins((bgm::BgmPlugin, spacial::SpacialAudioPlugin))
            .add_event::<PlaySound>()
            .add_systems(Update, (play_sounds,).run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Event)]
pub struct PlaySound {
    pub clip: Handle<AudioSource>,
    pub volume: f64,
    pub playback_rate: f64,
    pub rand_speed_intensity: f64,
    pub repeat: bool,
    pub parent: Option<Entity>,
}

impl Default for PlaySound {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            volume: 1.0,
            playback_rate: 1.0,
            rand_speed_intensity: 0.0,
            repeat: false,
            parent: None,
        }
    }
}

fn play_sounds(
    mut commands: Commands,
    audio: Res<Audio>,
    mut ev_play_sound: EventReader<PlaySound>,
) {
    let mut rng = thread_rng();
    let mut added_sounds: HashSet<Handle<AudioSource>> = HashSet::new();

    for ev in ev_play_sound.read() {
        if added_sounds.contains(&ev.clip) {
            continue;
        }
        added_sounds.insert(ev.clip.clone());

        let speed_offset = if ev.rand_speed_intensity == 0.0 {
            0.0
        } else {
            rng.gen_range(-1.0..1.0) * ev.rand_speed_intensity
        };
        let volume_offset = if ev.parent.is_some() { 0.0 } else { 1.0 };

        let audio_instance = if ev.repeat {
            audio
                .play(ev.clip.clone())
                .with_volume(ev.volume * volume_offset * MAIN_VOLUME)
                .with_playback_rate(ev.playback_rate + speed_offset)
                .looped()
                .handle()
        } else {
            audio
                .play(ev.clip.clone())
                .with_volume(ev.volume * volume_offset * MAIN_VOLUME)
                .with_playback_rate(ev.playback_rate + speed_offset)
                .handle()
        };

        if let Some(parent) = ev.parent {
            let audio_emitter = commands
                .spawn((
                    TransformBundle::default(),
                    AudioEmitter {
                        instances: vec![audio_instance],
                    },
                ))
                .id();
            commands.entity(parent).push_children(&[audio_emitter]);
        };
    }
}