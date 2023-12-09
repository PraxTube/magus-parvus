pub mod input;
pub mod state;
pub mod stats;

pub use state::PlayerChangedState;
pub use state::PlayerState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::ui::health::Health;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::camera::YSort;
use crate::world::game_entity::SpawnGameEntity;
use crate::world::CHUNK_SIZE;
use crate::{GameAssets, GameState};

use input::PlayerInput;
use stats::Stats;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(2.5 * CHUNK_SIZE, 16.0 + 2.5 * CHUNK_SIZE, 0.0);
const STAGGERING_TIME: f32 = 0.25;
const STAGGERING_INTENSITY: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_indicies,
                adjust_sprite_flip,
                player_movement,
                apply_contact_damage,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_plugins((input::InputPlugin, state::PlayerStatePlugin))
        .add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub collider_entity: Entity,
    pub staggering_timer: Timer,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
            staggering_timer: Timer::from_seconds(STAGGERING_TIME, TimerMode::Repeating),
        }
    }
}

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (0, 5),
        PlayerState::Moving => (6, 11),
        PlayerState::Casting => (12, 17),
        PlayerState::Staggering => (18, 18),
    }
}

fn update_indicies(mut q_player: Query<(&mut AnimationIndices, &mut TextureAtlasSprite, &Player)>) {
    let (mut indices, mut sprite, player) = match q_player.get_single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };

    let new_indices = player_sprite_indicies(&player.state);

    if new_indices.0 != indices.first {
        indices.first = new_indices.0;
        indices.last = new_indices.1;
        sprite.index = indices.first;
    }
}

fn adjust_sprite_flip(mut q_player: Query<(&mut TextureAtlasSprite, &Player)>) {
    let (mut sprite, player) = match q_player.get_single_mut() {
        Ok(p) => (p.0, p.1),
        Err(_) => return,
    };
    if player.current_direction.x == 0.0 {
        return;
    }

    sprite.flip_x = player.current_direction.x < 0.0;
}

pub fn player_movement(
    mut q_player: Query<(&mut Velocity, &mut Player, &Stats)>,
    player_input: Res<PlayerInput>,
) {
    let (mut velocity, mut player, stats) = match q_player.get_single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let direction = player_input.move_direction;
    if direction == Vec2::default() {
        player.state = PlayerState::Idling;
        velocity.linvel = Vec2::ZERO;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    velocity.linvel = direction * stats.move_speed;
}

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
        ))
        .id();

    let health = Health::new(entity, 10.0, 0.60);
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

fn apply_contact_damage(
    mut q_player: Query<(&mut Velocity, &mut Player, &mut Health, &Transform)>,
    q_enemies: Query<(&Transform, &Enemy), Without<Player>>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (mut velocity, mut player, mut health, player_transform) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let enemy_parent = if &player.collider_entity == source {
            match q_colliders.get(*target) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == target {
            match q_colliders.get(*source) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let (enemy_transform, enemy) = match q_enemies.get(enemy_parent.get()) {
            Ok(e) => (e.0, e.1),
            Err(_) => continue,
        };

        health.health -= enemy.damage;
        player.state = PlayerState::Staggering;

        let dir = (player_transform.translation - enemy_transform.translation)
            .truncate()
            .normalize_or_zero();
        // This makes the player look towards the impact
        player.current_direction = -dir;
        velocity.linvel = dir * STAGGERING_INTENSITY;
    }
}
