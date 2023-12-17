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
            state::DemonBossStatePlugin,
        ));
    }
}

const MOVE_SPEED: f32 = 50.0;

#[derive(Component)]
struct DemonBoss {
    state: DemonBossState,
}

impl Default for DemonBoss {
    fn default() -> Self {
        Self {
            state: DemonBossState::Idling,
        }
    }
}
