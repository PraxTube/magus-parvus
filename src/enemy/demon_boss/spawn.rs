use bevy::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{player::PLAYER_SPAWN_POS, GameAssets, GameState};

fn spawn_demon_boss(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.enemy_boss_animations[1].clone())
        .repeat();

    commands.spawn((
        animator,
        SpriteSheetBundle {
            texture_atlas: assets.enemy_boss.clone(),
            transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(Vec3::splat(2.0)),
            ..default()
        },
    ));
}

pub struct DemonBossSpawnPlugin;

impl Plugin for DemonBossSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_demon_boss,));
    }
}
