pub mod demon_boss;
pub mod slime;

use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((slime::EnemySlimePlugin, demon_boss::DemonBossPlugin));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub damage: f32,
}
