use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::PLAYER_SPAWN_POS, world::camera::YSort, GameAssets, GameState};

use super::{statue::StatueUnlockedDelayed, ActiveItems, STATUE_COUNT};

const OFFSET: Vec3 = Vec3::new(0.0, -24.0, 0.0);
const MAX_ITEM_HEIGHT: f32 = 16.0;
const ITEM_SPEED: f32 = 15.0;

#[derive(Component)]
struct Platform;
#[derive(Component, Default)]
struct PlatformItem {
    move_up: bool,
}
#[derive(Component)]
struct PlatformItemComponent;

fn spawn_platform(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.platform_animations[0].clone());

    commands.spawn((
        Platform,
        animator,
        YSort(-10.0),
        SpriteSheetBundle {
            texture_atlas: assets.platform.clone(),
            transform: Transform::from_translation(PLAYER_SPAWN_POS + OFFSET),
            ..default()
        },
    ));
}

fn trigger_platform(
    mut commands: Commands,
    assets: Res<GameAssets>,
    active_items: Res<ActiveItems>,
    mut q_platform: Query<&mut AnimationPlayer2D, With<Platform>>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    if active_items.len() < STATUE_COUNT {
        return;
    }

    commands.spawn((
        PlatformItem::default(),
        SpriteBundle {
            texture: assets.platform_item.clone(),
            transform: Transform::from_translation(PLAYER_SPAWN_POS + OFFSET),
            ..default()
        },
    ));
    commands.spawn((
        PlatformItemComponent,
        SpriteBundle {
            texture: assets.platform_item_shadow.clone(),
            transform: Transform::from_translation(
                PLAYER_SPAWN_POS + OFFSET + Vec3::new(0.0, 0.0, -4.0),
            ),
            ..default()
        },
    ));
    commands.spawn((
        PlatformItemComponent,
        SpriteBundle {
            texture: assets.platform_item_highlight.clone(),
            transform: Transform::from_translation(
                PLAYER_SPAWN_POS + OFFSET + Vec3::new(0.0, -OFFSET.y, -5.0),
            ),
            ..default()
        },
    ));

    let mut animator = match q_platform.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    animator.play(assets.platform_animations[1].clone());
}

fn hover_platform_item(
    time: Res<Time>,
    mut q_platform_item: Query<(&mut Transform, &mut PlatformItem)>,
) {
    let (mut transform, mut item) = match q_platform_item.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if transform.translation.y >= PLAYER_SPAWN_POS.y + MAX_ITEM_HEIGHT {
        item.move_up = false
    }
    if transform.translation.y <= PLAYER_SPAWN_POS.y - MAX_ITEM_HEIGHT {
        item.move_up = true;
    }

    let sign = if item.move_up { 1.0 } else { -1.0 };

    transform.translation += sign * Vec3::Y * ITEM_SPEED * time.delta_seconds();
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_platform)
            .add_systems(
                Update,
                (trigger_platform, hover_platform_item).run_if(in_state(GameState::Gaming)),
            );
    }
}
