use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::utils::quat_from_vec2;
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SPEED: f32 = 300.0;
const SCALE: f32 = 1.25;
const SCALE_TIME: f32 = 0.35;
const INFERNO_COUNT: usize = 25;

#[derive(Component)]
pub struct Fireball {
    disabled: bool,
    piercing: bool,
    pub damage: f32,
    timer: Timer,
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            disabled: false,
            piercing: false,
            damage: 5.0,
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

impl Fireball {
    pub fn disable(&mut self) {
        if !self.piercing {
            self.disabled = true;
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}

fn spawn_fireball(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    transform: Transform,
    damage: f32,
    piercing: bool,
) {
    let entity = commands
        .spawn((
            Fireball {
                damage,
                piercing,
                ..default()
            },
            AnimSprite::new(60, true),
            AnimSpriteTimer::new(0.05),
            Sprite::from_atlas_image(
                assets.fireball_texture.clone(),
                TextureAtlas {
                    layout: assets.fireball_layout.clone(),
                    ..default()
                },
            ),
            transform,
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(5.0),
            Sensor,
            Transform::from_translation(Vec3::new(-25.0, 0.0, 0.0)),
        ))
        .id();

    commands.entity(entity).add_children(&[collider]);
}

fn spawn_fireballs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    q_enemies: Query<&Transform, With<Enemy>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(p) => (p.0, p.1),
        Err(_) => return,
    };

    let mut closest_enemy = Vec2::INFINITY;
    for enemy_transform in &q_enemies {
        if enemy_transform.translation.truncate().length_squared() < closest_enemy.length_squared()
        {
            closest_enemy = enemy_transform.translation.truncate();
        }
    }
    let rot = if closest_enemy == Vec2::INFINITY {
        quat_from_vec2(Vec2::new(-player.current_direction.x, 0.0))
    } else {
        quat_from_vec2(-closest_enemy + player_transform.translation.truncate())
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Fireball {
            let transform = Transform::from_translation(player_transform.translation)
                .with_scale(Vec3::splat(SCALE))
                .with_rotation(rot);
            spawn_fireball(&mut commands, &assets, transform, 5.0, false);
        }
    }
}

fn spawn_ignis_pila(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    q_enemies: Query<&Transform, With<Enemy>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let (player_pos, player) = match q_player.get_single() {
        Ok(p) => (p.0.translation, p.1),
        Err(_) => return,
    };

    let mut closest_enemy = Vec2::INFINITY;
    for enemy_transform in &q_enemies {
        if enemy_transform.translation.truncate().length_squared() < closest_enemy.length_squared()
        {
            closest_enemy = enemy_transform.translation.truncate();
        }
    }
    let angle = if closest_enemy == Vec2::INFINITY {
        if player.current_direction.x < 0.0 {
            0.0
        } else {
            PI
        }
    } else {
        Vec2::X.angle_to(-closest_enemy + player_pos.truncate())
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::IgnisPila {
            for offset in [0.0, 0.08, -0.08, 0.16, -0.16] {
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(Quat::from_rotation_z(angle + offset));
                spawn_fireball(&mut commands, &assets, transform, 3.0, true);
            }
        }
    }
}

fn spawn_inferno_pila(
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
        if ev.spell == Spell::InfernoPila {
            for i in 0..INFERNO_COUNT {
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(Quat::from_rotation_z(TAU * i as f32 / INFERNO_COUNT as f32));
                spawn_fireball(&mut commands, &assets, transform, 2.0, true);
            }
        }
    }
}

fn scale_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        if transform.scale.x < SCALE {
            transform.scale += Vec3::ONE * SCALE / SCALE_TIME * time.delta_secs();
        }
    }
}

fn move_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        let dir = -transform.local_x();
        transform.translation += dir * SPEED * time.delta_secs();
    }
}

fn tick_fireball_timers(time: Res<Time>, mut q_fireballs: Query<&mut Fireball>) {
    for mut fireball in &mut q_fireballs {
        fireball.timer.tick(time.delta());
        if fireball.timer.just_finished() {
            fireball.disabled = true;
        }
    }
}

fn despawn_fireballs(mut commands: Commands, q_fireballs: Query<(Entity, &Fireball)>) {
    for (entity, fireball) in &q_fireballs {
        if fireball.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fireballs,
                spawn_ignis_pila,
                spawn_inferno_pila,
                despawn_fireballs,
                scale_fireballs,
                move_fireballs,
                tick_fireball_timers,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
