use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::audio::PlaySound;
use crate::player::Player;
use crate::utils::quat_from_vec2;
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SPEED: f32 = 300.0;
const SCALE: f32 = 1.5;
const MAX_PLAYER_DISTANCE: f32 = 500.0;
const ATTACK_TIME: f32 = 1.0;
const STRIKE_INTERVALS: [f32; 10] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.85, 0.9, 1.0, 1.1, 1.2];
const RAND_STRIKE_RADIUS: f32 = 25.0;

#[derive(Component)]
struct LightningBird {
    disabled: bool,
    attack_timer: Timer,
    flap_timer: Timer,
}

#[derive(Component)]
pub struct LightningStrike {
    pub damage: f32,
    timer: Timer,
}

#[derive(Component)]
struct LightningStrikeSpawnTimer {
    pos: Vec3,
    timer: Timer,
}

impl Default for LightningBird {
    fn default() -> Self {
        Self {
            disabled: false,
            attack_timer: Timer::from_seconds(ATTACK_TIME, TimerMode::Once),
            flap_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
        }
    }
}

impl Default for LightningStrike {
    fn default() -> Self {
        Self {
            damage: 5.0,
            timer: Timer::from_seconds(0.6, TimerMode::Repeating),
        }
    }
}

fn spawn_lightning_bird(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let mut animation_player = AnimationPlayer2D::default();
    animation_player
        .play(assets.lightning_bird_animations[1].clone())
        .repeat();

    commands.spawn((
        LightningBird::default(),
        animation_player,
        SpriteSheetBundle {
            transform,
            texture_atlas: assets.lightning_bird.clone(),
            ..default()
        },
    ));
}

fn spawn_lightning_birds(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(p) => (p.0, p.1),
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::FulgurAvis {
            let transform = Transform::from_translation(player_transform.translation)
                .with_scale(Vec3::splat(SCALE))
                .with_rotation(quat_from_vec2(player.current_direction));
            spawn_lightning_bird(&mut commands, &assets, transform);
        }
    }
}

fn spawn_flap_sounds(
    assets: Res<GameAssets>,
    q_lightning_birds: Query<(Entity, &LightningBird)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, lightning_bird) in &q_lightning_birds {
        if lightning_bird.flap_timer.just_finished() {
            ev_play_sound.send(PlaySound {
                clip: assets.lightning_bird_flap_sound.clone(),
                parent: Some(entity),
                ..default()
            });
        }
    }
}

fn spawn_lightning_strike_spawn_timers(
    mut commands: Commands,
    q_lightning_birds: Query<(&Transform, &LightningBird)>,
) {
    let mut rng = thread_rng();

    for (transform, lightning_bird) in &q_lightning_birds {
        if !lightning_bird.attack_timer.just_finished() {
            continue;
        }

        for time in STRIKE_INTERVALS {
            let rand_offset = RAND_STRIKE_RADIUS
                * Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            let speed_offset = transform.local_x() * time * SPEED;
            let pos = transform.translation + rand_offset + speed_offset;
            commands.spawn(LightningStrikeSpawnTimer {
                pos,
                timer: Timer::from_seconds(time, TimerMode::Once),
            });
        }
    }
}

fn spawn_lightning_strikes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_lightning_strike_timers: Query<(Entity, &mut LightningStrikeSpawnTimer)>,
) {
    for (entity, mut strike) in &mut q_lightning_strike_timers {
        strike.timer.tick(time.delta());
        if !strike.timer.just_finished() {
            continue;
        }

        let collider = commands
            .spawn((
                Collider::ball(25.0),
                Sensor,
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -24.0, 0.0,
                ))),
            ))
            .id();
        let mut animation_player = AnimationPlayer2D::default();
        animation_player.play(assets.lightning_strike_animations[0].clone());

        commands
            .spawn((
                LightningStrike::default(),
                animation_player,
                SpriteSheetBundle {
                    transform: Transform::from_translation(strike.pos),
                    texture_atlas: assets.lightning_strike.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);
        commands.entity(entity).despawn_recursive();
    }
}

fn tick_timers(time: Res<Time>, mut q_lightning_birds: Query<&mut LightningBird>) {
    for mut lightning_bird in &mut q_lightning_birds {
        lightning_bird.flap_timer.tick(time.delta());
        lightning_bird.attack_timer.tick(time.delta());
    }
}

fn move_lightning_birds(
    time: Res<Time>,
    mut q_lightning_birds: Query<&mut Transform, With<LightningBird>>,
) {
    for mut transform in &mut q_lightning_birds {
        let dir = transform.local_x();
        transform.translation += dir * SPEED * time.delta_seconds();
    }
}

fn rotate_lightning_birds(
    q_player: Query<&Transform, With<Player>>,
    mut q_lightning_birds: Query<(&mut Transform, &mut LightningBird), Without<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation.truncate(),
        Err(_) => return,
    };

    for (mut transform, mut lightning_bird) in &mut q_lightning_birds {
        let dis = player_pos.distance_squared(transform.translation.truncate());
        if dis > MAX_PLAYER_DISTANCE.powi(2) {
            transform.rotation = quat_from_vec2(player_pos - transform.translation.truncate());
            lightning_bird.attack_timer.reset();
        }
    }
}

fn adjust_sprite_flip(
    mut q_lightning_birds: Query<(&mut TextureAtlasSprite, &Transform), With<LightningBird>>,
) {
    for (mut sprite, transform) in &mut q_lightning_birds {
        sprite.flip_y = transform.rotation.to_euler(EulerRot::ZYX).0.abs() > PI / 2.0;
    }
}

fn despawn_lightning_birds(
    mut commands: Commands,
    q_lightning_birds: Query<(Entity, &LightningBird)>,
) {
    for (entity, fireball) in &q_lightning_birds {
        if fireball.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_lightning_strikes(
    mut commands: Commands,
    time: Res<Time>,
    mut q_lightning_strikes: Query<(Entity, &mut LightningStrike)>,
) {
    for (entity, mut lightning_strike) in &mut q_lightning_strikes {
        lightning_strike.timer.tick(time.delta());
        if lightning_strike.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct LightningBirdPlugin;

impl Plugin for LightningBirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_lightning_birds,
                spawn_flap_sounds,
                spawn_lightning_strike_spawn_timers,
                spawn_lightning_strikes,
                tick_timers,
                move_lightning_birds,
                rotate_lightning_birds,
                adjust_sprite_flip,
                despawn_lightning_birds,
                despawn_lightning_strikes,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
