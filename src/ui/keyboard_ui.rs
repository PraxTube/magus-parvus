use bevy::prelude::*;

use crate::player::PLAYER_SPAWN_POS;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::{GameAssets, GameState};

const ANCHOR: Vec3 = Vec3::new(-150.0, 100.0, -1.0);
const BLINK_TIME: f32 = 0.5;

enum Icon {
    J,
    K,
    A,
    D,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn icon_index(icon: Icon) -> usize {
    match icon {
        Icon::J => 0,
        Icon::DOWN => 2,
        Icon::K => 3,
        Icon::UP => 5,
        Icon::A => 6,
        Icon::LEFT => 8,
        Icon::D => 9,
        Icon::RIGHT => 11,
    }
}

fn spawn_icon(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    icon: Icon,
    offset: Vec2,
    animated: bool,
) {
    let index = icon_index(icon);

    if !(animated) {
        commands.spawn((SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite { index, ..default() },
            transform: Transform::from_translation(offset.extend(0.0) + ANCHOR + PLAYER_SPAWN_POS)
                .with_scale(Vec3::splat(0.75)),
            ..default()
        },));
        return;
    }

    commands.spawn((
        AnimationIndices {
            first: index,
            last: index + 1,
        },
        FrameTimer(Timer::from_seconds(BLINK_TIME, TimerMode::Repeating)),
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite { index, ..default() },
            transform: Transform::from_translation(offset.extend(0.0) + ANCHOR + PLAYER_SPAWN_POS)
                .with_scale(Vec3::splat(0.75)),
            ..default()
        },
    ));
}

fn spawn_keyboard_ui(mut commands: Commands, assets: Res<GameAssets>) {
    let texture_atlas = assets.keyboard_ui.clone();
    let button_dis = 30.0;
    let arrow_dis = 55.0;

    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::J,
        Vec2::new(0.0, -button_dis),
        true,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::DOWN,
        Vec2::new(0.0, -arrow_dis),
        false,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::K,
        Vec2::new(0.0, button_dis),
        true,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::UP,
        Vec2::new(0.0, arrow_dis),
        false,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::A,
        Vec2::new(-button_dis, 0.0),
        true,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::LEFT,
        Vec2::new(-arrow_dis, 0.0),
        false,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::D,
        Vec2::new(button_dis, 0.0),
        true,
    );
    spawn_icon(
        &mut commands,
        texture_atlas.clone(),
        Icon::RIGHT,
        Vec2::new(arrow_dis, 0.0),
        false,
    );
}

pub struct KeyboardUiPlugin;

impl Plugin for KeyboardUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_keyboard_ui);
    }
}
