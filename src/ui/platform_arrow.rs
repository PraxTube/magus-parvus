use bevy::prelude::*;

use crate::{
    item::{statue::StatueUnlockedDelayed, ActiveItems, STATUE_COUNT},
    player::{Player, PLAYER_SPAWN_POS},
    utils::quat_from_vec2,
    world::{camera_shake::update_camera, MainCamera},
    GameAssets, GameState,
};

const OFFSET: f32 = 150.0;

#[derive(Component)]
struct Arrow;

fn spawn_arrow(
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

    commands.spawn((Arrow, Sprite::from_image(assets.platform_arrow.clone())));
}

fn despawn_arrow(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_arrow: Query<Entity, With<Arrow>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    if player_pos
        .truncate()
        .distance_squared(PLAYER_SPAWN_POS.truncate())
        > OFFSET.powi(2)
    {
        return;
    }

    for entity in &q_arrow {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_arrow(
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_arrow: Query<&mut Transform, (With<Arrow>, Without<MainCamera>)>,
) {
    let camera_pos = match q_camera.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };
    let mut transform = match q_arrow.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let dir = (PLAYER_SPAWN_POS - camera_pos)
        .truncate()
        .normalize_or_zero();

    transform.rotation = quat_from_vec2(dir);
    transform.translation = camera_pos + dir.extend(0.0) * OFFSET;
}

pub struct PlatformArrowPlugin;

impl Plugin for PlatformArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_arrow, despawn_arrow).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(
            PostUpdate,
            (update_arrow,)
                .before(update_camera)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
