use std::time::Duration;

use bevy::prelude::*;

use crate::player::stats::Stats;
use crate::player::Player;
use crate::GameState;

use super::{Spell, SpellCasted};

#[derive(Component)]
struct SpeedBoostTimer {
    timer: Timer,
}

impl Default for SpeedBoostTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(30.0), TimerMode::Once),
        }
    }
}

fn activate_speed_boost(
    mut commands: Commands,
    mut q_player: Query<&mut Stats, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let mut player_stats = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::SpeedBoost {
            player_stats.move_speed = 5.0 * Stats::default().move_speed;
            commands.spawn(SpeedBoostTimer::default());
        };
    }
}

fn deactivate_speed_boost(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<&mut Stats, With<Player>>,
    mut q_timers: Query<(Entity, &mut SpeedBoostTimer)>,
) {
    let mut player_stats = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let deactivate = q_timers.iter().count() == 1;

    for (entity, mut timer) in &mut q_timers {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            if deactivate {
                player_stats.move_speed = Stats::default().move_speed;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct SpeedBoostPlugin;

impl Plugin for SpeedBoostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (activate_speed_boost, deactivate_speed_boost).run_if(in_state(GameState::Gaming)),
        );
    }
}
