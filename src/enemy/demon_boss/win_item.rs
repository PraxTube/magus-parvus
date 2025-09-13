use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::{spawn::DemonBossDeath, DemonBoss};

#[derive(Component)]
struct WinItem;

#[derive(Component)]
struct WinItemDelay {
    timer: Timer,
    pos: Vec3,
}

fn spawn_win_item(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_delay: Query<(Entity, &mut WinItemDelay)>,
) {
    let (entity, mut delay) = match q_delay.single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    delay.timer.tick(time.delta());
    if !delay.timer.just_finished() {
        return;
    }
    let pos = delay.pos;
    commands.entity(entity).despawn();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.portal_animations[0].clone());

    commands.spawn((
        WinItem,
        Collider::cuboid(8.0, 21.0),
        Sensor,
        animator,
        YSort(0.0),
        Sprite::from_atlas_image(
            assets.portal_texture.clone(),
            TextureAtlas {
                layout: assets.portal_layout.clone(),
                ..default()
            },
        ),
        Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
    ));
}

fn spawn_win_item_delay(
    mut commands: Commands,
    q_demon_boss: Query<&Transform, With<DemonBoss>>,
    mut ev_demon_boss_death: EventReader<DemonBossDeath>,
) {
    if ev_demon_boss_death.is_empty() {
        return;
    }
    ev_demon_boss_death.clear();

    let pos = match q_demon_boss.single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    commands.spawn(WinItemDelay {
        timer: Timer::from_seconds(1.5, TimerMode::Once),
        pos,
    });
}

fn switch_animations(
    assets: Res<GameAssets>,
    mut q_win_item: Query<&mut AnimationPlayer2D, With<WinItem>>,
) {
    let mut animator = match q_win_item.single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if animator.finished() {
        animator.play(assets.portal_animations[1].clone()).repeat();
    }
}

fn adjust_flip(
    q_player: Query<&Transform, With<Player>>,
    mut q_win_item: Query<(&Transform, &mut Sprite), (With<WinItem>, Without<Player>)>,
) {
    let player_pos = match q_player.single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };
    let (pos, mut sprite) = match q_win_item.single_mut() {
        Ok(r) => (r.0.translation, r.1),
        Err(_) => return,
    };

    sprite.flip_x = pos.x > player_pos.x;
}

fn trigger_win(
    mut next_state: ResMut<NextState<GameState>>,
    mut q_player: Query<&Player>,
    q_win_item: Query<Entity, With<WinItem>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let player = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let item_entity = match q_win_item.single() {
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

        next_state.set(GameState::Win);
    }
}

pub struct WinItemPlugin;

impl Plugin for WinItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_win_item,
                spawn_win_item_delay,
                switch_animations,
                adjust_flip,
                trigger_win,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
