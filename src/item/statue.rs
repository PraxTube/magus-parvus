use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::{YSort, TRANSLATION_TO_PIXEL};
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::{GameAssets, GameState};

use super::Item;

const BLINK_OFFSET: Vec3 = Vec3::new(0.0, 32.0, 0.0);
const BEAM_OFFSET: Vec3 = Vec3::new(1.0, 45.0, -10.0);
const TRIGGER_DISTANCE_SQRT: f32 = 64.0 * 64.0;

#[derive(Component, Clone)]
pub struct Statue {
    pub item: Item,
    unlocked: bool,
}

impl Statue {
    pub fn new(item: Item) -> Self {
        Self {
            item,
            unlocked: false,
        }
    }
}

#[derive(Component)]
struct UnlockTimer {
    timer: Timer,
    ev: StatueUnlocked,
    disabled: bool,
}

impl UnlockTimer {
    fn new(ev: StatueUnlocked) -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            ev,
            disabled: false,
        }
    }
}

#[derive(Event, Clone)]
pub struct StatueUnlocked {
    pub statue: Statue,
    pos: Vec3,
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct StatueUnlockedDelayed(pub StatueUnlocked);

fn spawn_statues(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(Entity, &Item, &GridCoords), Without<Statue>>,
) {
    for (entity, item, grid_coords) in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        );

        let collider = commands
            .spawn((
                Collider::cuboid(20.0, 10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -20.0, 0.0,
                ))),
            ))
            .id();

        commands
            .entity(entity)
            .insert(YSort(0.0 + BACKGROUND_ZINDEX_ABS))
            .insert(Statue::new(item.clone()))
            .insert(SpriteBundle {
                texture: assets.statue.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            })
            .push_children(&[collider]);
    }
}

fn spawn_statue_blinks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    for ev in ev_statue_unlocked.read() {
        commands.spawn((
            AnimSprite::new(5, false),
            AnimSpriteTimer::new(0.1),
            YSort(10.0),
            SpriteSheetBundle {
                texture_atlas: assets.statue_blink.clone(),
                transform: Transform::from_translation(ev.pos + BLINK_OFFSET),
                ..default()
            },
        ));
    }
}

fn spawn_statue_beams(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_unlocked_delayed: EventReader<StatueUnlockedDelayed>,
) {
    for ev in ev_statue_unlocked_delayed.read() {
        commands.spawn((
            AnimSprite::new(4, true),
            AnimSpriteTimer::new(0.05),
            YSort((BEAM_OFFSET.y - 1.0) * TRANSLATION_TO_PIXEL),
            SpriteSheetBundle {
                texture_atlas: assets.statue_beam.clone(),
                transform: Transform::from_translation(ev.pos + BEAM_OFFSET),
                ..default()
            },
        ));
    }
}

fn spawn_unlock_timers(
    mut commands: Commands,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    for ev in ev_statue_unlocked.read() {
        commands.spawn(UnlockTimer::new(ev.clone()));
    }
}

fn despawn_unlock_timers(
    mut commands: Commands,
    q_unlock_timers: Query<(Entity, &UnlockTimer)>,
    mut ev_statue_unlocked_delayed: EventWriter<StatueUnlockedDelayed>,
) {
    for (entity, unlock_timer) in &q_unlock_timers {
        if unlock_timer.disabled {
            ev_statue_unlocked_delayed.send(StatueUnlockedDelayed(unlock_timer.ev.clone()));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn tick_unlock_timers(time: Res<Time>, mut q_unlock_timers: Query<&mut UnlockTimer>) {
    for mut unlock_timer in &mut q_unlock_timers {
        unlock_timer.timer.tick(time.delta());

        if unlock_timer.timer.just_finished() {
            unlock_timer.disabled = true;
        }
    }
}

fn unlock_statues(
    q_player: Query<&Transform, (With<Player>, Without<Statue>)>,
    mut q_statues: Query<(&GlobalTransform, &mut Statue)>,
    mut ev_statue_unlocked: EventWriter<StatueUnlocked>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for (statue_transform, mut statue) in &mut q_statues {
        if statue.unlocked {
            continue;
        }

        let dis = player_pos.distance_squared(statue_transform.translation());
        if dis <= TRIGGER_DISTANCE_SQRT {
            statue.unlocked = true;
            ev_statue_unlocked.send(StatueUnlocked {
                statue: statue.clone(),
                pos: statue_transform.translation(),
            })
        }
    }
}

pub struct StatuePlugin;

impl Plugin for StatuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_statues,
                spawn_statue_blinks,
                spawn_statue_beams,
                spawn_unlock_timers,
                tick_unlock_timers,
                despawn_unlock_timers,
                unlock_statues,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<StatueUnlocked>()
        .add_event::<StatueUnlockedDelayed>();
    }
}
