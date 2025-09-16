use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ui::health::{Health, SpawnPlayerHearts};
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::audio::StepsTimer;
use super::stats::Stats;
use super::{Player, PLAYER_HEALTH, PLAYER_SPAWN_POS};

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_player_hears: EventWriter<SpawnPlayerHearts>,
) {
    let entity = commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Stats::default(),
            Health::new(PLAYER_HEALTH),
            YSort(0.0),
            AnimationIndices { first: 0, last: 5 },
            FrameTimer(Timer::from_seconds(0.085, TimerMode::Repeating)),
            StepsTimer::default(),
            Sprite::from_atlas_image(
                assets.player_texture.clone(),
                TextureAtlas::from(assets.player_layout.clone()),
            ),
            Transform::from_translation(PLAYER_SPAWN_POS).with_scale(Vec3::splat(2.0)),
        ))
        .id();

    ev_spawn_player_hears.write(SpawnPlayerHearts {
        count: PLAYER_HEALTH as usize,
    });

    let collider = commands
        .spawn((
            Collider::ball(4.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            // CollisionGroups::new(
            //     Group::from_bits(0b1100).unwrap(),
            //     Group::from_bits(0b1100).unwrap(),
            // ),
            Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Player::new(collider))
        .add_children(&[collider]);
}

fn despawn_player(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    q_player: Query<(Entity, &Health), With<Player>>,
) {
    let (entity, health) = match q_player.single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if health.health <= 0.0 {
        commands.entity(entity).despawn();
        next_state.set(GameState::GameOver);
    }
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player)
            .add_systems(Update, despawn_player.run_if(in_state(GameState::Gaming)));
    }
}
