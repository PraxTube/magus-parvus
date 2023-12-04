use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SCUTUM_GLACIEI_COUNT: usize = 6;
const DISTANCE_FROM_PLAYER: f32 = 75.0;
const DELTA_ROTATION: f32 = TAU / 4.0;

#[derive(Component)]
pub struct Icicle {
    pub disabled: bool,
    pub damage: f32,
}

impl Default for Icicle {
    fn default() -> Self {
        Self {
            disabled: false,
            damage: 1.0,
        }
    }
}

const SCALE: f32 = 1.5;

fn spawn_icicle(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let entity = commands
        .spawn((
            Icicle::default(),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.icicle.clone(),
                ..default()
            },
            AnimSprite::new(30, true),
            AnimSpriteTimer::new(0.05),
            YSort(10.0),
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
            (spawn_scutum_glaciei, move_icicles, rotate_icicles)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
