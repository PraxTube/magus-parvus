use bevy::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::{Player, PlayerState};

const TIME_BETWEEN_STEPS: f32 = 0.3;
const RAND_SPEED_INTENSITY: f64 = 0.2;

#[derive(Component, Deref, DerefMut)]
pub struct StepsTimer(Timer);

impl Default for StepsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TIME_BETWEEN_STEPS,
            TimerMode::Repeating,
        ))
    }
}

fn tick_steps_timers(time: Res<Time>, mut q_steps_timer: Query<&mut StepsTimer>) {
    match q_steps_timer.get_single_mut() {
        Ok(mut t) => t.tick(time.delta()),
        Err(_) => return,
    };
}

fn play_step_sounds(
    assets: Res<GameAssets>,
    q_player: Query<&Player>,
    q_steps_timer: Query<&StepsTimer>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    let steps_timer = match q_steps_timer.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    if player_state != PlayerState::Moving {
        return;
    }
    if !steps_timer.just_finished() {
        return;
    }

    ev_play_sound.send(PlaySound {
        clip: assets.player_footstep.clone(),
        volume: 1.5,
        rand_speed_intensity: RAND_SPEED_INTENSITY,
        ..default()
    });
}

pub struct PlayerAudioPlugin;

impl Plugin for PlayerAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_step_sounds, tick_steps_timers).run_if(in_state(GameState::Gaming)),
        );
    }
}
