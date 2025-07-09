use bevy::prelude::*;

use crate::{
    item::{statue::StatueUnlockedDelayed, ActiveItems, STATUE_COUNT, statue::Statue},
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
    q_arrow: Query<(), With<Arrow>>,
) {
    if ev_statue_unlocked_delayed.is_empty() {
        return;
    }
    ev_statue_unlocked_delayed.clear();

    // 确保不重复生成箭头
    let arrow_exists = !q_arrow.is_empty();
    
    // 还有未激活的雕像时才显示箭头
    if !arrow_exists && active_items.len() < STATUE_COUNT {
        commands.spawn((
            Arrow,
            SpriteBundle {
                texture: assets.platform_arrow.clone(),
                ..default()
            },
        ));
    }
}


fn despawn_arrow(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_arrow: Query<Entity, With<Arrow>>,
    active_items: Res<ActiveItems>,
) {
    // 如果所有雕像都被激活，则移除箭头
    if active_items.len() >= STATUE_COUNT {
        for entity in &q_arrow {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }
    
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
    q_statues: Query<(&GlobalTransform, &Statue)>,
    active_items: Res<ActiveItems>,
) {
    let camera_pos = match q_camera.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };
    
    let mut transform = match q_arrow.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    // 查找下一个未激活的雕像
    let next_statue_pos = find_next_statue_pos(&q_statues, &active_items);
    
    if let Some(next_pos) = next_statue_pos {
        let dir = (next_pos - camera_pos)
           .truncate()
           .normalize_or_zero();

        transform.rotation = quat_from_vec2(dir);
        transform.translation = camera_pos + dir.extend(0.0) * OFFSET;
    }
}

// 查找下一个未激活的雕像的位置
fn find_next_statue_pos(q_statues: &Query<(&GlobalTransform, &Statue)>, active_items: &Res<ActiveItems>) -> Option<Vec3> {
    // 遍历所有雕像，找到第一个不在 active_items 中的雕像
    for (statue_transform, statue) in q_statues.iter() {
        if !active_items.contains(&statue.item) {
            return Some(statue_transform.translation());
        }
    }
    None
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
