use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::item::statue::StatueUnlockedDelayed;
use crate::item::{ActiveItems, STATUE_COUNT};
use crate::player::PLAYER_SPAWN_POS;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::{GameAssets, GameState};

const ANCHOR: Vec3 = Vec3::new(-150.0, 100.0, -1.0);
const BLINK_TIME: f32 = 0.5;

#[derive(Component)]
struct KeyboardIcon;

enum Icon {
    J,
    K,
    A,
    D,
    I,
    H,
    Left,
    Right,
    Up,
    Down,
}

fn icon_index(icon: Icon) -> usize {
    match icon {
        Icon::J => 0,
        Icon::Down => 2,
        Icon::K => 3,
        Icon::Up => 5,
        Icon::A => 6,
        Icon::Left => 8,
        Icon::D => 9,
        Icon::Right => 11,
        Icon::I => 12,
        Icon::H => 15,
    }
}

fn spawn_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    icon: Icon,
    offset: Vec2,
    animated: bool,
) {
    let index = icon_index(icon);

    let mut icon = commands.spawn((
        KeyboardIcon,
        SpriteBundle {
            texture: assets.keyboard_ui_texture.clone(),
            transform: Transform::from_translation(offset.extend(0.0) + ANCHOR + PLAYER_SPAWN_POS)
                .with_scale(Vec3::splat(0.75)),
            ..default()
        },
        TextureAtlas {
            layout: assets.keyboard_ui_layout.clone(),
            index,
        },
    ));

    if animated {
        icon.insert((
            Collider::cuboid(16.0, 16.0),
            AnimationIndices {
                first: index,
                last: index + 1,
            },
            FrameTimer(Timer::from_seconds(BLINK_TIME, TimerMode::Repeating)),
        ));
    }
}

fn spawn_keyboard_ui(mut commands: Commands, assets: Res<GameAssets>) {
    let button_dis = 30.0;
    let arrow_dis = 55.0;

    spawn_icon(
        &mut commands,
        &assets,
        Icon::J,
        Vec2::new(0.0, -button_dis),
        true,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::Down,
        Vec2::new(0.0, -arrow_dis),
        false,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::K,
        Vec2::new(0.0, button_dis),
        true,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::Up,
        Vec2::new(0.0, arrow_dis),
        false,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::A,
        Vec2::new(-button_dis, 0.0),
        true,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::Left,
        Vec2::new(-arrow_dis, 0.0),
        false,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::D,
        Vec2::new(button_dis, 0.0),
        true,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::Right,
        Vec2::new(arrow_dis, 0.0),
        false,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::I,
        Vec2::new(400.0, 150.0),
        true,
    );
    spawn_icon(
        &mut commands,
        &assets,
        Icon::H,
        Vec2::new(30.0, 800.0),
        true,
    );
}

fn despawn_keyboard_ui(
    mut commands: Commands,
    active_items: Res<ActiveItems>,
    q_icons: Query<Entity, With<KeyboardIcon>>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    if active_items.len() < STATUE_COUNT {
        return;
    }

    for entity in &q_icons {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct KeyboardUiPlugin;

impl Plugin for KeyboardUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_keyboard_ui)
            .add_systems(
                Update,
                (despawn_keyboard_ui,).run_if(in_state(GameState::Gaming)),
            );
    }
}
