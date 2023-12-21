use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    enemy::demon_boss::spawn::DemonBossDeath,
    item::{platform::TriggerFinalAct, statue::StatueUnlockedDelayed},
    GameAssets, GameState,
};

use super::GameAudio;

const BGM_VOLUME: f64 = 0.5;

#[derive(Component)]
struct Bgm {
    handle: Handle<AudioInstance>,
}

#[derive(Component, Deref, DerefMut)]
struct UnmuteTimer(Timer);

impl Default for UnmuteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
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

fn play_boss_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    let volume = game_audio.main_volume * BGM_VOLUME;
    let handle = audio
        .play(assets.bgm_boss.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(3.0),
            AudioEasing::InPowi(3),
        ))
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

fn mute_bgms(
    mut commands: Commands,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<&Bgm>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            commands.spawn(UnmuteTimer::default());
            instance.set_volume(
                0.0,
                AudioTween::new(Duration::from_secs_f32(0.5), AudioEasing::Linear),
            );
        }
    }
}

fn unmute_bgms(
    mut commands: Commands,
    time: Res<Time>,
    game_audio: Res<GameAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut q_unmute_timers: Query<(Entity, &mut UnmuteTimer)>,
    q_bgms: Query<&Bgm>,
) {
    let mut unmute = false;
    for (entity, mut unmute_timer) in &mut q_unmute_timers {
        unmute_timer.tick(time.delta());
        if unmute_timer.just_finished() {
            unmute = true;
            commands.entity(entity).despawn_recursive();
        }
    }

    if !unmute {
        return;
    }

    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            instance.set_volume(
                game_audio.main_volume * BGM_VOLUME,
                AudioTween::new(Duration::from_secs_f32(5.0), AudioEasing::InPowi(2)),
            );
        }
    }
}

fn despawn_normal_bgm(
    mut commands: Commands,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<(Entity, &Bgm)>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    for (entity, bgm) in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            instance.stop(AudioTween::default());
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn fade_out_boss_bgm(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<&Bgm>,
    mut ev_demon_boss_death: EventReader<DemonBossDeath>,
) {
    if ev_demon_boss_death.is_empty() {
        return;
    }
    ev_demon_boss_death.clear();

    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            instance.set_volume(
                0.0,
                AudioTween::new(Duration::from_secs_f32(2.5), AudioEasing::Linear),
            );
        }
    }
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), play_bgm)
            .add_systems(
                Update,
                (
                    play_boss_bgm.run_if(in_state(GameState::Gaming)),
                    update_bgm_volumes.run_if(in_state(GameState::GameOver)),
                    mute_bgms.after(update_bgm_volumes),
                    unmute_bgms,
                    despawn_normal_bgm,
                    fade_out_boss_bgm,
                ),
            );
    }
}
