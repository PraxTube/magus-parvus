use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ui::health::Health;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::camera::YSort;
use crate::world::game_entity::SpawnGameEntity;
use crate::{GameAssets, GameState};

use super::audio::StepsTimer;
use super::stats::Stats;
use super::{Player, PLAYER_SPAWN_POS};

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_game_entity: EventWriter<SpawnGameEntity>,
) {
    let entity = commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Stats::default(),
            YSort(0.0),
            AnimationIndices { first: 0, last: 5 },
            FrameTimer(Timer::from_seconds(0.085, TimerMode::Repeating)),
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS)
                    .with_scale(Vec3::splat(2.0)),
                texture_atlas: assets.player.clone(),
                ..default()
            },
            StepsTimer::default(),
        ))
        .id();

    let health = Health::new(entity, 10.0, 2.00);
    ev_spawn_game_entity.send(SpawnGameEntity { entity, health });

    let collider = commands
        .spawn((
            Collider::ball(4.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            // CollisionGroups::new(
            //     Group::from_bits(0b1100).unwrap(),
            //     Group::from_bits(0b1100).unwrap(),
            // ),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Player::new(collider))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
