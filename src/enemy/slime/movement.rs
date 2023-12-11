use rand::{self, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Enemy, SlimeEnemy, SlimeState, JUMP_TIME, MAX_JUMP_SPEED, RANDOM_OFFSET_INTENSITY};
use crate::player::Player;

fn update_jump_position(
    mut q_slimes: Query<(&Transform, &mut SlimeEnemy), (With<Enemy>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    for (enemy_transform, mut slime) in &mut q_slimes {
        if slime.state == SlimeState::Jumping {
            continue;
        }

        let distance = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());
        let ratio = (distance / MAX_JUMP_SPEED / JUMP_TIME).min(1.0);

        let mut rng = rand::thread_rng();
        let dir = (player_transform.translation.truncate()
            - enemy_transform.translation.truncate())
        .normalize_or_zero();
        let random_offset =
            Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)) * RANDOM_OFFSET_INTENSITY;

        slime.jump_speed = ratio * MAX_JUMP_SPEED;
        slime.jump_direction = dir + random_offset;
    }
}

fn move_slimes(mut q_slimes: Query<(&mut Velocity, &SlimeEnemy)>) {
    for (mut velocity, slime) in &mut q_slimes {
        if slime.state == SlimeState::Staggering {
            continue;
        }
        if slime.state != SlimeState::Jumping {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        velocity.linvel = slime.jump_direction * slime.jump_speed;
    }
}

pub struct SlimeMovementPlugin;

impl Plugin for SlimeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_jump_position, move_slimes));
    }
}
