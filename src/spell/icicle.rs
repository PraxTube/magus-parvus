use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SCALE: f32 = 1.5;
const SCUTUM_GLACIEI_COUNT: usize = 10;
const DISTANCE_FROM_PLAYER: f32 = 75.0;
const DELTA_ROTATION: f32 = TAU / 4.0;
const TIME: f32 = 10.0;

#[derive(Component)]
pub struct Icicle {
    timer: Timer,
    pub disabled: bool,
    pub damage: f32,
}

impl Default for Icicle {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(TIME, TimerMode::Once),
            disabled: false,
            damage: 1.0,
        }
    }
}

fn tick_timers(time: Res<Time>, mut q_icicles: Query<&mut Icicle>) {
    for mut icicle in &mut q_icicles {
        icicle.timer.tick(time.delta());
        if icicle.timer.just_finished() {
            icicle.disabled = true;
        }
    }
}

fn spawn_icicle(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let entity = commands
        .spawn((
            Icicle::default(),
            YSort(10.0),
            AnimSprite::new(30, true),
            AnimSpriteTimer::new(0.05),
            SpriteBundle {
                texture: assets.icicle_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.icicle_layout.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(25.0, 4.0),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(5.0, 0.0, 0.0))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}

fn spawn_scutum_glaciei(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::ScutumGlaciei {
            for i in 0..SCUTUM_GLACIEI_COUNT {
                let rot = Quat::from_rotation_z(TAU * i as f32 / SCUTUM_GLACIEI_COUNT as f32);
                let transform = Transform::from_translation(
                    player_pos + rot.mul_vec3(Vec3::X) * DISTANCE_FROM_PLAYER,
                )
                .with_scale(Vec3::splat(SCALE))
                .with_rotation(rot);
                spawn_icicle(&mut commands, &assets, transform);
            }
        }
    }
}

fn spawn_icicle_shatter(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec3) {
    commands.spawn((
        YSort(10.0),
        AnimSprite::new(49, false),
        AnimSpriteTimer::new(0.02),
        SpriteBundle {
            texture: assets.icicle_shatter_texture.clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        TextureAtlas {
            layout: assets.icicle_shatter_layout.clone(),
            ..default()
        },
    ));
}

fn despawn_icicles(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_icicles: Query<(Entity, &Transform, &Icicle)>,
) {
    for (entity, transform, icicle) in &q_icicles {
        if icicle.disabled {
            spawn_icicle_shatter(&mut commands, &assets, transform.translation);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_icicles(
    q_player: Query<&Transform, With<Player>>,
    mut q_icicles: Query<&mut Transform, (With<Icicle>, Without<Player>)>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for mut transform in &mut q_icicles {
        let offset = transform.rotation.mul_vec3(Vec3::X) * DISTANCE_FROM_PLAYER;
        transform.translation = player_pos + offset;
    }
}

fn rotate_icicles(time: Res<Time>, mut q_icicles: Query<&mut Transform, With<Icicle>>) {
    for mut transform in &mut q_icicles {
        transform.rotate_z(DELTA_ROTATION * time.delta_seconds());
    }
}

pub struct IciclePlugin;

impl Plugin for IciclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_timers,
                spawn_scutum_glaciei,
                despawn_icicles,
                move_icicles,
                rotate_icicles,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
