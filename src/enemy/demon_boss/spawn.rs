use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{
    enemy::Enemy,
    player::PLAYER_SPAWN_POS,
    ui::health::Health,
    world::camera::{YSort, TRANSLATION_TO_PIXEL},
    GameAssets, GameState,
};

use super::{
    audio::DemonBossStepsTimer, state::DemonBossState, strike::DemonBossStrike, DemonBoss,
};

const SCALE: f32 = 1.5;

#[derive(Component)]
struct Shadow;
#[derive(Component)]
struct DemonCollider;

fn spawn_demon_boss(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.enemy_boss_animations[0].clone())
        .repeat();

    let shadow = commands
        .spawn((
            Shadow,
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
            DemonCollider,
            Collider::ball(25.0),
            CollisionGroups::default(),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle::from(Transform::from_translation(Vec3::new(0.0, -30.0, 0.0))),
        ))
        .id();

    commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            DemonBoss::default(),
            DemonBossStepsTimer::default(),
            DemonBossStrike::default(),
            Enemy { damage: 0.0 },
            Health::new(10.0),
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
        .push_children(&[shadow, collider]);
}

fn despawn_demon_boss(
    mut commands: Commands,
    mut q_demon_boss: Query<(Entity, &Health, &mut DemonBoss, &AnimationPlayer2D)>,
) {
    let (entity, health, mut demon_boss, animator) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Dying && animator.is_finished() {
        commands.entity(entity).despawn_recursive();
    }

    if health.health <= 0.0 && demon_boss.state != DemonBossState::Dying {
        demon_boss.state = DemonBossState::Dying;
    }
}

fn despawn_shadow(
    time: Res<Time>,
    q_demon_boss: Query<&DemonBoss>,
    mut q_shadow: Query<&mut Sprite, With<Shadow>>,
) {
    let demon_boss = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut sprite = match q_shadow.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Dying {
        return;
    }

    sprite.color = Color::rgba(
        sprite.color.r(),
        sprite.color.g(),
        sprite.color.b(),
        sprite.color.a() - time.delta_seconds(),
    );
}

fn despawn_demon_colliders(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_demon_colliders: Query<Entity, With<DemonCollider>>,
) {
    let demon_boss = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Dying {
        return;
    }

    for entity in &q_demon_colliders {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct DemonBossSpawnPlugin;

impl Plugin for DemonBossSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_demon_boss,))
            .add_systems(
                Update,
                (despawn_demon_boss, despawn_shadow, despawn_demon_colliders),
            );
    }
}
