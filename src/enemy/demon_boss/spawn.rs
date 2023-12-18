use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{
    player::PLAYER_SPAWN_POS,
    world::camera::{YSort, TRANSLATION_TO_PIXEL},
    GameAssets, GameState,
};

use super::{attack::StrikeHitbox, audio::DemonBossStepsTimer, DemonBoss};

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
                transform: Transform::from_translation(Vec3::new(0.0, -44.0, 0.0)),
                ..default()
            },
        ))
        .id();
    let collider = commands
        .spawn((
            Collider::ball(25.0),
            CollisionGroups::default(),
            TransformBundle::from(Transform::from_translation(Vec3::new(0.0, -30.0, 0.0))),
        ))
        .id();
    let strike_hitbox = commands
        .spawn((
            StrikeHitbox,
            Sensor,
            Collider::cuboid(25.0, 15.0),
            CollisionGroups::default(),
            TransformBundle::default(),
        ))
        .id();

    commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            DemonBoss::default(),
            DemonBossStepsTimer::default(),
            animator,
            YSort(36.0 * SCALE * TRANSLATION_TO_PIXEL),
            SpriteSheetBundle {
                texture_atlas: assets.enemy_boss.clone(),
                transform: Transform::from_translation(
                    PLAYER_SPAWN_POS - Vec3::new(0.0, 100.0, 0.0),
                )
                .with_scale(Vec3::splat(SCALE)),
                ..default()
            },
        ))
        .push_children(&[shadow, collider, strike_hitbox]);
}

pub struct DemonBossSpawnPlugin;

impl Plugin for DemonBossSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_demon_boss,));
    }
}
