mod attack;
mod audio;
mod movement;
mod spawn;
mod state;

use bevy::prelude::*;

use state::DemonBossState;

pub struct DemonBossPlugin;

impl Plugin for DemonBossPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::DemonBossSpawnPlugin,
            movement::DemonBossMovementPlugin,
            attack::DemonBossAttackPlugin,
            state::DemonBossStatePlugin,
            audio::DemonBossAudioPlugin,
        ));
    }
}

const MOVE_SPEED: f32 = 50.0;
const STRIKE_RANGE: f32 = 150.0;

#[derive(Component)]
pub struct DemonBoss {
    pub damage: f32,
    state: DemonBossState,
}

impl Default for DemonBoss {
    fn default() -> Self {
        Self {
            damage: 1.0,
            state: DemonBossState::Idling,
        }
    }
}
