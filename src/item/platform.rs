use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    audio::PlaySound,
    player::{Player, PLAYER_SPAWN_POS},
    world::camera::YSort,
    GameAssets, GameState,
};

use super::{statue::StatueUnlockedDelayed, ActiveItems, STATUE_COUNT};

const OFFSET: Vec3 = Vec3::new(0.0, -16.0, 0.0);
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

#[derive(Event)]
pub struct TriggerFinalAct;

fn spawn_platform(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.platform_animations[0].clone());

    commands.spawn((
        Platform,
        animator,
        YSort(-10.0),
        SpriteBundle {
            texture: assets.platform_texture.clone(),
            transform: Transform::from_translation(PLAYER_SPAWN_POS + OFFSET),
            ..default()
        },
        TextureAtlas {
            layout: assets.platform_layout.clone(),
            ..default()
        },
    ));
}

fn spawn_trigger_item(
    mut commands: Commands,
    assets: Res<GameAssets>,
    active_items: Res<ActiveItems>,
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
        Sensor,
        Collider::ball(15.0),
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
}

fn despawn_trigger_item(
    mut commands: Commands,
    q_platform_items: Query<Entity, With<PlatformItem>>,
    q_platform_components: Query<Entity, With<PlatformItemComponent>>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    for entity in &q_platform_items {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &q_platform_components {
        commands.entity(entity).despawn_recursive();
    }
}

fn trigger_platform(
    assets: Res<GameAssets>,
    mut q_platform: Query<&mut AnimationPlayer2D, With<Platform>>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    let mut animator = match q_platform.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    animator.play(assets.platform_animations[1].clone());
}

fn play_trigger_sound(
    assets: Res<GameAssets>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    ev_play_sound.send(PlaySound {
        clip: assets.item_unlock_sound.clone(),
        reverse: true,
        ..default()
    });
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

fn trigger_final_act(
    mut q_player: Query<&Player>,
    q_final_act: Query<Entity, With<PlatformItem>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    mut ev_trigger_final_act: EventWriter<TriggerFinalAct>,
) {
    let player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let item_entity = match q_final_act.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        if !(&player.collider_entity == source && &item_entity == target
            || &player.collider_entity == target && &item_entity == source)
        {
            continue;
        }

        ev_trigger_final_act.send(TriggerFinalAct);
    }
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_trigger_item,
                despawn_trigger_item,
                hover_platform_item,
                play_trigger_sound,
                trigger_platform,
                trigger_final_act,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<TriggerFinalAct>()
        .add_systems(OnEnter(GameState::Gaming), spawn_platform);
    }
}
