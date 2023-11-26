use bevy::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::utils::quat_from_vec2;
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

#[derive(Component)]
struct Fireball;

const SPEED: f32 = 300.0;
const SCALE: f32 = 1.5;
const SCALE_TIME: f32 = 0.35;

fn spawn_fireball(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    commands.spawn((
        Fireball,
        SpriteSheetBundle {
            transform,
            texture_atlas: assets.fireball.clone(),
            ..default()
        },
        AnimSprite::new(60, true),
        AnimSpriteTimer::new(0.05),
    ));
}

fn spawn_fireballs(
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
        if ev.spell == Spell::Fireball {
            let transform = Transform::from_translation(player_transform.translation)
                .with_scale(Vec3::splat(SCALE))
                .with_rotation(quat_from_vec2(-player.current_direction));
            spawn_fireball(&mut commands, &assets, transform);
        }
    }
}

fn spawn_fireball_circles(
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
        if ev.spell == Spell::FireballCircle {
            for dir in [
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(-1.0, 1.0),
            ] {
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(quat_from_vec2(dir));
                spawn_fireball(&mut commands, &assets, transform);
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(quat_from_vec2(-dir));
                spawn_fireball(&mut commands, &assets, transform);
            }
        }
    }
}

fn scale_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        if transform.scale.x < SCALE {
            transform.scale += Vec3::ONE * SCALE / SCALE_TIME * time.delta_seconds();
        }
    }
}

fn move_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        let dir = -transform.local_x();
        transform.translation += dir * SPEED * time.delta_seconds();
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fireballs.run_if(in_state(GameState::Gaming)),
                spawn_fireball_circles.run_if(in_state(GameState::Gaming)),
                scale_fireballs,
                move_fireballs,
            ),
        )
        .add_event::<SpellCasted>();
    }
}
