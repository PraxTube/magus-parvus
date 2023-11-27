use bevy::prelude::*;

use crate::{GameAssets, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_enemies);
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_enemies(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Enemy,
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(32.0 * 32.0, 32.0 * 32.0, 0.0))
                .with_scale(Vec3::splat(1.5)),
            texture_atlas: assets.enemy.clone(),
            ..default()
        },
    ));
}
