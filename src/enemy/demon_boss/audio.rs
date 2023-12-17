use bevy::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::{DemonBoss, DemonBossState};

const TIME_BETWEEN_STEPS: f32 = 0.8;
const RAND_SPEED_INTENSITY: f64 = 0.2;

#[derive(Component, Deref, DerefMut)]
pub struct DemonBossStepsTimer(Timer);

impl Default for DemonBossStepsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TIME_BETWEEN_STEPS,
            TimerMode::Repeating,
        ))
    }
}

fn tick_timers(time: Res<Time>, mut q_steps_timer: Query<&mut DemonBossStepsTimer>) {
    match q_steps_timer.get_single_mut() {
        Ok(mut t) => t.tick(time.delta()),
        Err(_) => return,
    };
}

fn play_step_sounds(
    assets: Res<GameAssets>,
    q_demon_boss: Query<(Entity, &DemonBoss)>,
    q_steps_timer: Query<&DemonBossStepsTimer>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let (entity, demon_boss_state) = match q_demon_boss.get_single() {
        Ok(r) => (r.0, r.1.state),
        Err(_) => return,
    };
    let steps_timer = match q_steps_timer.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss_state != DemonBossState::Moving {
        return;
    }
    if !steps_timer.just_finished() {
        return;
    }

    ev_play_sound.send(PlaySound {
        clip: assets.demon_boss_step_sound.clone(),
        rand_speed_intensity: RAND_SPEED_INTENSITY,
        parent: Some(entity),
        ..default()
    });
}

pub struct DemonBossAudioPlugin;

impl Plugin for DemonBossAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_step_sounds, tick_timers).run_if(in_state(GameState::Gaming)),
        );
    }
}
