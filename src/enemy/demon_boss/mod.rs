mod spawn;

use bevy::prelude::*;

pub struct DemonBossPlugin;

impl Plugin for DemonBossPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::DemonBossSpawnPlugin);
    }
}
