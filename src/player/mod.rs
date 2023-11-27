pub mod input;
pub mod stats;

use bevy::prelude::*;

use crate::{GameAssets, GameState};

use self::{input::PlayerInput, stats::Stats};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_indicies,
                animate_sprite,
                adjust_sprite_flip,
                player_movement,
                switch_player_mode,
                signal_player_state_change,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_plugins(input::InputPlugin)
        .add_event::<PlayerChangedState>()
        .add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct FrameTimer(Timer);

#[derive(Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    Casting,
}

#[derive(Component, Default)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
}

#[derive(Event)]
pub struct PlayerChangedState {
    pub old_state: PlayerState,
    pub new_state: PlayerState,
}

fn signal_player_state_change(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state != *old_state {
        ev_changed_state.send(PlayerChangedState {
            old_state: *old_state,
            new_state: player.state,
        });
        *old_state = player.state;
    }
}

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (0, 5),
        PlayerState::Moving => (6, 11),
        PlayerState::Casting => (12, 17),
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

fn animate_sprite(
    time: Res<Time>,
    mut q_player: Query<
        (&AnimationIndices, &mut FrameTimer, &mut TextureAtlasSprite),
        With<Player>,
    >,
) {
    let (indices, mut timer, mut sprite) = match q_player.get_single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };

    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = if sprite.index == indices.last {
            indices.first
        } else {
            sprite.index + 1
        };
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
    time: Res<Time>,
    mut q_player: Query<(&mut Transform, &mut Player, &Stats)>,
    player_input: Res<PlayerInput>,
) {
    let (mut transform, mut player, stats) = match q_player.get_single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let direction = player_input.move_direction;
    if direction == Vec2::default() {
        player.state = PlayerState::Idling;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    transform.translation += direction.extend(0.0) * stats.move_speed * time.delta_seconds();
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Player::default(),
        Stats::default(),
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(32.0 * 32.0, 32.0 * 32.0, 0.0))
                .with_scale(Vec3::splat(2.0)),
            texture_atlas: assets.player.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        AnimationIndices { first: 0, last: 5 },
        FrameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn switch_player_mode(keys: Res<Input<KeyCode>>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    match player.state {
        PlayerState::Idling => {
            if keys.just_pressed(KeyCode::I) {
                player.state = PlayerState::Casting;
            }
        }
        PlayerState::Moving => {
            if keys.just_pressed(KeyCode::I) {
                player.state = PlayerState::Casting;
            }
        }
        PlayerState::Casting => {
            if keys.just_pressed(KeyCode::Escape) {
                player.state = PlayerState::Idling;
            }
        }
    }
}
