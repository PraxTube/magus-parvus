use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
struct GameOverScreen;

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

fn spawn_quit_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS Q TO QUIT".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
    let winner_text = spawn_title(commands, font.clone());
    let quit_text = spawn_quit_text(commands, font.clone());

    commands
        .spawn((
            GameOverScreen,
            NodeBundle {
                style: Style {
                    top: Val::Percent(20.0),
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
        .push_children(&[winner_text, quit_text]);
}

fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(&mut commands, assets.font.clone());
}

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), (spawn_game_over_screen,));
    }
}
