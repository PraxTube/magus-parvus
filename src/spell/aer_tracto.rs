use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const AER_TRACTO_COUNT: usize = 15;
const OFFEST: Vec3 = Vec3::new(150.0, 0.0, 0.0);
const SCALE: f32 = 2.0;
const PULL_INTENSITY: f32 = 300.0;

#[derive(Component)]
pub struct AerTracto {
    pub pull_intensity: f32,
}

impl Default for AerTracto {
    fn default() -> Self {
        Self {
            pull_intensity: PULL_INTENSITY,
        }
    }
}

fn spawn_air_pull(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let entity = commands
        .spawn((
            AerTracto::default(),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.aer_tracto.clone(),
                ..default()
            },
            AnimSprite::new(7, false),
            AnimSpriteTimer::new(0.12),
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
                let offset = rot.mul_vec3(OFFEST);
                let transform = Transform::from_translation(player_pos + offset)
                    .with_rotation(rot)
                    .with_scale(Vec3::splat(SCALE));
                spawn_air_pull(&mut commands, &assets, transform);
            }
        }
    }
}

pub struct AerTractoPlugin;

impl Plugin for AerTractoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_aer_tracto,).run_if(in_state(GameState::Gaming)),
        );
    }
}
