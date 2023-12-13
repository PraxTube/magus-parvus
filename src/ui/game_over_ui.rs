use bevy::prelude::*;

use crate::{audio::GameAudio, GameAssets, GameState};

const AUDIO_SILENCE_TIME: f64 = 3.0;

#[derive(Component)]
struct GameOverScreen;

#[derive(Component, Deref, DerefMut)]
struct AudioSilenceTimer(Timer);

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) {
    commands.spawn((
        GameOverScreen,
        ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                texture,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.85)),
            z_index: ZIndex::Local(100),
            ..default()
        },
    ));
}

fn spawn_title(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "GAME OVER".to_string(),
        text_style.clone(),
    )]);
    commands.spawn((GameOverScreen, text_bundle)).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
    let title_text = spawn_title(commands, font.clone());

    commands
        .spawn((
            GameOverScreen,
            NodeBundle {
                style: Style {
                    top: Val::Percent(35.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(10.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[title_text]);
}

fn spawn_audio_silence_timer(commands: &mut Commands) {
    commands.spawn(AudioSilenceTimer(Timer::from_seconds(
        AUDIO_SILENCE_TIME as f32,
        TimerMode::Once,
    )));
}

fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(&mut commands, assets.font.clone());
    spawn_audio_silence_timer(&mut commands);
}

fn reduce_audio_volume(
    mut commands: Commands,
    time: Res<Time>,
    mut game_audio: ResMut<GameAudio>,
    mut q_audio_silence_timer: Query<(Entity, &mut AudioSilenceTimer)>,
) {
    let (entity, mut timer) = match q_audio_silence_timer.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    timer.tick(time.delta());
    if timer.just_finished() {
        commands.entity(entity).despawn_recursive();
    }

    game_audio.main_volume =
        (game_audio.main_volume - 1.0 / AUDIO_SILENCE_TIME * time.delta_seconds_f64()).max(0.0);
}

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), (spawn_game_over_screen,))
            .add_systems(
                Update,
                (reduce_audio_volume,).run_if(in_state(GameState::GameOver)),
            );
    }
}
