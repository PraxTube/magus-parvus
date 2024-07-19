use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SCALE: f32 = 2.0;
const PULL_INTENSITY: f32 = 400.0;
const PUSH_INTENSITY: f32 = 400.0;

const AER_TRACTO_COUNT: usize = 15;
const AER_PELLO_COUNT: usize = 10;
const AER_TRACTO_OFFSET: Vec3 = Vec3::new(125.0, 0.0, 0.0);
const AER_PELLO_OFFSET: Vec3 = Vec3::new(75.0, 0.0, 0.0);

#[derive(Component)]
pub struct AerTracto {
    pub damage: f32,
    pub pull_intensity: f32,
}

impl AerTracto {
    fn new(pull_intensity: f32) -> Self {
        Self {
            damage: 1.0,
            pull_intensity,
        }
    }
}

fn spawn_air_pull(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    transform: Transform,
    pull_intensity: f32,
) {
    let entity = commands
        .spawn((
            AerTracto::new(pull_intensity),
            AnimSprite::new(7, false),
            AnimSpriteTimer::new(0.10),
            SpriteBundle {
                texture: assets.aer_tracto_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.aer_tracto_layout.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(15.0),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}

fn spawn_aer_tracto(
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
        if ev.spell == Spell::AerTracto {
            for i in 0..AER_TRACTO_COUNT {
                let rot = Quat::from_rotation_z(TAU * i as f32 / AER_TRACTO_COUNT as f32);
                let offset = rot.mul_vec3(AER_TRACTO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(SCALE));
                spawn_air_pull(&mut commands, &assets, transform, PULL_INTENSITY);

                let local_scale = 0.5;
                let offset = rot.mul_vec3(local_scale * AER_TRACTO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(local_scale * SCALE));
                spawn_air_pull(
                    &mut commands,
                    &assets,
                    transform,
                    PULL_INTENSITY * local_scale,
                );

                let local_scale = 1.5;
                let offset = rot.mul_vec3(local_scale * AER_TRACTO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(local_scale * SCALE));
                spawn_air_pull(
                    &mut commands,
                    &assets,
                    transform,
                    PULL_INTENSITY * local_scale,
                );
            }
        }
    }
}

fn spawn_aer_pello(
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
        if ev.spell == Spell::AerPello {
            for i in 0..AER_PELLO_COUNT {
                let rot = Quat::from_rotation_z(TAU * i as f32 / AER_PELLO_COUNT as f32);
                let offset = rot.mul_vec3(-AER_PELLO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(SCALE));
                spawn_air_pull(&mut commands, &assets, transform, PUSH_INTENSITY);

                let local_scale = 0.5;
                let offset = rot.mul_vec3(-local_scale * AER_PELLO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(local_scale * SCALE));
                spawn_air_pull(
                    &mut commands,
                    &assets,
                    transform,
                    PUSH_INTENSITY * local_scale,
                );

                let local_scale = 1.5;
                let offset = rot.mul_vec3(-local_scale * AER_PELLO_OFFSET);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(local_scale * SCALE));
                spawn_air_pull(
                    &mut commands,
                    &assets,
                    transform,
                    PUSH_INTENSITY * local_scale,
                );
            }
        }
    }
}

pub struct AerTractoPlugin;

impl Plugin for AerTractoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_aer_tracto, spawn_aer_pello).run_if(in_state(GameState::Gaming)),
        );
    }
}
