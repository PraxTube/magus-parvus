use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Enemy, SlimeEnemy, SlimeState, SpawnSlimeEnemy};
use crate::audio::PlaySound;
use crate::ui::health::Health;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

fn spawn_slime(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    spawn_pos: Vec3,
    ev_play_sound: &mut EventWriter<PlaySound>,
) {
    let entity = commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Enemy { damage: 1.0 },
            SlimeEnemy::default(),
            AnimationIndices { first: 0, last: 5 },
            FrameTimer(Timer::from_seconds(0.085, TimerMode::Repeating)),
            YSort(0.0),
            SpriteSheetBundle {
                transform: Transform::from_translation(spawn_pos).with_scale(Vec3::splat(1.5)),
                texture_atlas: assets.enemy.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            // CollisionGroups::new(
            //     Group::from_bits(0b0100).unwrap(),
            //     Group::from_bits(0b1000).unwrap(),
            // ),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Health::new(entity, 10.0, 0.60))
        .push_children(&[collider]);

    ev_play_sound.send(PlaySound {
        clip: assets.slime_land_sound.clone(),
        rand_speed_intensity: 0.2,
        parent: Some(entity),
        ..default()
    });
}

fn spawn_slimes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_slime_enemy: EventReader<SpawnSlimeEnemy>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for ev in ev_spawn_slime_enemy.read() {
        spawn_slime(&mut commands, &assets, ev.pos, &mut ev_play_sound);
    }
}

fn despawn_slimes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut q_slimes: Query<(Entity, &Health, &mut SlimeEnemy)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, health, mut slime) in &mut q_slimes {
        if health.health <= 0.0 && slime.state != SlimeState::Dying {
            ev_play_sound.send(PlaySound {
                clip: assets.slime_death_sound.clone(),
                parent: Some(entity),
                ..default()
            });
            slime.state = SlimeState::Dying;
        }
        if slime.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct SlimeSpawnPlugin;

impl Plugin for SlimeSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_slimes, despawn_slimes).run_if(in_state(GameState::Gaming)),
        );
    }
}
