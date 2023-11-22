mod ui;
mod world;

use std::time::Duration;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Gaming),
        )
        .add_event::<PlayerChangedState>()
        .add_plugins((ui::UiPlugin, world::WorldPlugin))
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                update_indicies,
                animate_sprite,
                adjust_sprite_flip,
                player_movement,
                switch_player_mode,
                cast_spell,
                signal_player_state_change,
                double_j_escape,
            ),
        )
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct FrameTimer(Timer);

#[derive(Default, PartialEq, Clone, Copy)]
enum PlayerState {
    #[default]
    Idling,
    Moving,
    Casting,
}

#[derive(Component, Default)]
struct Player {
    state: PlayerState,
    current_direction: Vec2,
}

#[derive(Event)]
pub struct PlayerChangedState {
    old_state: PlayerState,
    new_state: PlayerState,
}

fn signal_player_state_change(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = q_player.single();

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
    let (mut indices, mut sprite, player) = q_player.single_mut();
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
    let (indices, mut timer, mut sprite) = q_player.single_mut();

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
    let (mut sprite, player) = q_player.single_mut();
    if player.current_direction.x == 0.0 {
        return;
    }

    sprite.flip_x = player.current_direction.x < 0.0;
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_player: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = q_player.single_mut();
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let mut direction = Vec2::default();
    if keys.pressed(KeyCode::J) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::K) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::F) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::A) {
        direction += Vec2::new(-1.0, 0.0);
    }
    let direction = direction.normalize_or_zero();

    if direction == Vec2::default() {
        player.state = PlayerState::Idling;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    let speed = 300.0;
    transform.translation += direction.extend(0.0) * speed * time.delta_seconds();
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("mage.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 6, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        Player::default(),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationIndices { first: 0, last: 5 },
        FrameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn switch_player_mode(keys: Res<Input<KeyCode>>, mut q_player: Query<&mut Player>) {
    let mut player = q_player.single_mut();

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

fn double_j_escape(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_player: Query<&mut Player>,
    mut timer: Local<Timer>,
) {
    let mut player = q_player.single_mut();
    let duration = Duration::from_secs_f32(0.2);

    if player.state != PlayerState::Casting {
        timer.set_elapsed(duration);
        return;
    }

    timer.tick(time.delta());
    if keys.just_pressed(KeyCode::J) {
        if timer.finished() {
            timer.set_duration(duration);
            timer.reset();
            return;
        }

        player.state = PlayerState::Idling;
    }
}

fn cast_spell(
    keys: Res<Input<KeyCode>>,
    q_player: Query<&Player>,
    mut string: Local<String>,
    mut ev_received_char: EventReader<ReceivedCharacter>,
) {
    let player = q_player.single();
    if player.state != PlayerState::Casting {
        return;
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", &*string);
        string.clear();
    }
    if keys.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in ev_received_char.read() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
}
