pub mod explosion;

mod audio;
mod cast;
mod movement;
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
        ));
    }
}

const MOVE_SPEED: f32 = 50.0;
const STRIKE_RANGE: f32 = 150.0;
const INV_CAST_RANGE: f32 = 400.0;

#[derive(Component)]
pub struct DemonBoss {
    pub damage: f32,
    state: DemonBossState,
}

impl Default for DemonBoss {
    fn default() -> Self {
        Self {
            damage: 999.0,
            state: DemonBossState::Idling,
        }
    }
}
