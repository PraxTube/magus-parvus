use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{
    enemy::Enemy,
    item::platform::TriggerFinalAct,
    player::PLAYER_SPAWN_POS,
    ui::health::Health,
    world::camera::{YSort, TRANSLATION_TO_PIXEL},
    GameAssets, GameState,
};

use super::{
    audio::DemonBossStepsTimer, state::DemonBossState, strike::DemonBossStrike, DemonBoss,
};

const SCALE: f32 = 1.5;
const SPAWN_POS: Vec3 = Vec3::new(200.0, 0.0, 0.0);

#[derive(Component)]
struct Shadow;
#[derive(Component)]
struct DemonCollider;

#[derive(Component, Deref, DerefMut)]
struct SpawnDelay(Timer);
#[derive(Component)]
struct Smoke;

#[derive(Event)]
pub struct DemonBossDeath;

fn spawn_smokes(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.demon_boss_smoke_animations[0].clone());

    commands.spawn((
        Smoke,
        animator,
        YSort(1.0),
        SpriteSheetBundle {
            texture_atlas: assets.demon_boss_smoke.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(5.0)),
            ..default()
        },
    ));
}

fn spawn_demon_boss(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    q_smoke: Query<Entity, (With<Smoke>, Without<SpawnDelay>)>,
    mut q_delay: Query<(Entity, &mut SpawnDelay)>,
) {
    let (entity, mut delay) = match q_delay.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    delay.tick(time.delta());
    if !delay.just_finished() {
        return;
    }
    commands.entity(entity).despawn_recursive();

    let pos = PLAYER_SPAWN_POS + SPAWN_POS;
    if q_smoke.is_empty() {
        spawn_smokes(&mut commands, &assets, pos);
        commands.spawn(SpawnDelay(Timer::from_seconds(0.2, TimerMode::Once)));
        return;
    }

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
            Health::new(100.0),
            animator,
            YSort(36.0 * SCALE * TRANSLATION_TO_PIXEL),
            SpriteSheetBundle {
                texture_atlas: assets.enemy_boss.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(SCALE)),
                ..default()
            },
        ))
        .push_children(&[shadow, collider]);
}

fn spawn_demon_boss_delay(
    mut commands: Commands,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    commands.spawn(SpawnDelay(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn despawn_demon_boss(
    mut commands: Commands,
    mut q_demon_boss: Query<(Entity, &Health, &mut DemonBoss, &AnimationPlayer2D)>,
    mut ev_demon_boss_death: EventWriter<DemonBossDeath>,
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
        ev_demon_boss_death.send(DemonBossDeath);
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
        app.add_systems(
            Update,
            (
                spawn_demon_boss,
                spawn_demon_boss_delay,
                despawn_demon_boss,
                despawn_shadow,
                despawn_demon_colliders,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<DemonBossDeath>();
    }
}
