use bevy::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{
    player::PLAYER_SPAWN_POS,
    world::camera::{YSort, TRANSLATION_TO_PIXEL},
    GameAssets, GameState,
};

const SCALE: f32 = 1.5;

fn spawn_demon_boss(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.enemy_boss_animations[0].clone())
        .repeat();

    let shadow = commands
        .spawn((
            YSort(-1.0),
            SpriteBundle {
                texture: assets.enemy_boss_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -80.0, 0.0)),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            animator,
            YSort(72.0 * SCALE * TRANSLATION_TO_PIXEL),
            SpriteSheetBundle {
                texture_atlas: assets.enemy_boss.clone(),
                transform: Transform::from_translation(PLAYER_SPAWN_POS)
                    .with_scale(Vec3::splat(SCALE)),
                ..default()
            },
        ))
        .push_children(&[shadow]);
}

pub struct DemonBossSpawnPlugin;

impl Plugin for DemonBossSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_demon_boss,));
    }
}
