pub mod explosion;

mod audio;
mod cast;
mod collision;
mod earth_prison;
mod movement;
mod rage;
mod spawn;
mod state;
mod strike;

use bevy::prelude::*;

use state::DemonBossState;

pub struct DemonBossPlugin;

impl Plugin for DemonBossPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::DemonBossSpawnPlugin,
            movement::DemonBossMovementPlugin,
            strike::DemonBossAttackPlugin,
            state::DemonBossStatePlugin,
            audio::DemonBossAudioPlugin,
            cast::DemonBossCastPlugin,
            explosion::DemonBossExplosionPlugin,
            collision::DemonBossCollisionPlugin,
            rage::DemonBossRagePlugin,
            earth_prison::DemonBossEarthPrisonPlugin,
        ));
    }
}

const MOVE_SPEED: f32 = 50.0;
const STRIKE_RANGE: f32 = 125.0;
const INV_CAST_RANGE: f32 = 300.0;

#[derive(Component)]
pub struct DemonBoss {
    pub damage: f32,
    rage: DemonBossRage,
    state: DemonBossState,
}

#[derive(Component)]
pub struct DemonBossRage {
    active: bool,
    rage_stack: usize,
    timer: Timer,
}

impl DemonBossRage {
    pub fn add(&mut self) {
        self.rage_stack += 1;
        if self.rage_stack == 5 {
            self.active = true;
            self.rage_stack = 0;
        }
    }
}

impl Default for DemonBoss {
    fn default() -> Self {
        Self {
            damage: 2.0,
            rage: DemonBossRage {
                active: false,
                rage_stack: 0,
                timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            },
            state: DemonBossState::Idling,
        }
    }
}
