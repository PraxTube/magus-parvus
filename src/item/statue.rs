use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::ui::world_text::{SpawnWorldText, WorldText};
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::{YSort, TRANSLATION_TO_PIXEL};
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::{GameAssets, GameState};

use super::platform::TriggerFinalAct;
use super::{ActiveItems, Item};

const BLINK_OFFSET: Vec3 = Vec3::new(0.0, 32.0, 0.0);
const BEAM_OFFSET: Vec3 = Vec3::new(1.0, 45.0, -10.0);
const TRIGGER_DISTANCE_SQRT: f32 = 64.0 * 64.0;

#[derive(Component, Clone, Default)]
pub struct Statue {
    pub item: Item,
    pub pos: Vec3,
    triggered: bool,
    pub all_enemies_spawned: bool,
    unlocked: bool,
}
#[derive(Component)]
struct StatueBeam;

#[derive(Component)]
struct UnlockTimer {
    timer: Timer,
    ev: StatueUnlocked,
    disabled: bool,
}

#[derive(Event, Clone)]
pub struct StatueTriggered {
    pub statue: Statue,
}

#[derive(Event, Clone)]
pub struct StatueUnlocked {
    pub statue: Statue,
}

#[derive(Event, Clone, Deref, DerefMut)]
pub struct StatueUnlockedDelayed(pub StatueUnlocked);

impl Statue {
    pub fn new(item: Item) -> Self {
        Self {
            item,
            pos: Vec3::ZERO,
            triggered: false,
            all_enemies_spawned: false,
            unlocked: false,
        }
    }

    // 添加公共方法来获取 unlocked 字段的值
    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    // 添加公共方法来设置 unlocked 字段的值
    pub fn set_unlocked(&mut self, unlocked: bool) {
        self.unlocked = unlocked;
    }
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

fn spawn_statue_trigger_marks(
    mut ev_statue_triggered: EventReader<StatueTriggered>,
    mut ev_spawn_world_text: EventWriter<SpawnWorldText>,
) {
    for ev in ev_statue_triggered.read() {
        let world_text = WorldText {
            font_scale: 15.0,
            offset: Vec3::new(0.0, 35.0, 10.0),
            ..default()
        };
        ev_spawn_world_text.send(SpawnWorldText {
            world_text,
            pos: ev.statue.pos,
            content: "!".to_string(),
        });
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
            SpriteBundle {
                texture: assets.statue_blink_texture.clone(),
                transform: Transform::from_translation(ev.statue.pos + BLINK_OFFSET),
                ..default()
            },
            TextureAtlas {
                layout: assets.statue_blink_layout.clone(),
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
            StatueBeam,
            AnimSprite::new(4, true),
            AnimSpriteTimer::new(0.05),
            YSort((BEAM_OFFSET.y - 1.0) * TRANSLATION_TO_PIXEL),
            SpriteBundle {
                texture: assets.statue_beam_texture.clone(),
                transform: Transform::from_translation(ev.statue.pos + BEAM_OFFSET),
                ..default()
            },
            TextureAtlas {
                layout: assets.statue_beam_layout.clone(),
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

fn trigger_statues(
    q_player: Query<&Transform, (With<Player>, Without<Statue>)>,
    mut q_statues: Query<(&GlobalTransform, &mut Statue)>,
    mut ev_statue_triggered: EventWriter<StatueTriggered>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for (statue_transform, mut statue) in &mut q_statues {
        if statue.triggered {
            continue;
        }

        let dis = player_pos.distance_squared(statue_transform.translation());
        if dis > TRIGGER_DISTANCE_SQRT {
            continue;
        }

        statue.triggered = true;
        statue.pos = statue_transform.translation();
        ev_statue_triggered.send(StatueTriggered {
            statue: statue.clone(),
        });
    }
}

fn unlock_statues(
    mut active_items: ResMut<ActiveItems>,
    q_enemies: Query<&Enemy>,
    mut q_statues: Query<&mut Statue>,
    mut ev_statue_unlocked: EventWriter<StatueUnlocked>,
) {
    for mut statue in &mut q_statues {
        if statue.is_unlocked() {
            continue;
        }
        if !statue.all_enemies_spawned {
            continue;
        }

        if !q_enemies.is_empty() {
            continue;
        }

        statue.set_unlocked(true);
        active_items.push(statue.item.clone());
        ev_statue_unlocked.send(StatueUnlocked {
            statue: statue.clone(),
        });
    }
}

fn despawn_statues(
    mut commands: Commands,
    q_statues: Query<Entity, With<Statue>>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    for entity in &q_statues {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_statue_beams(
    mut commands: Commands,
    q_statue_beams: Query<Entity, With<StatueBeam>>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    for entity in &q_statue_beams {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct StatuePlugin;

impl Plugin for StatuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_statues,
                spawn_statue_trigger_marks,
                spawn_statue_blinks,
                spawn_statue_beams,
                spawn_unlock_timers,
                tick_unlock_timers,
                despawn_unlock_timers,
                trigger_statues,
                unlock_statues,
                despawn_statues,
                despawn_statue_beams,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<StatueTriggered>()
        .add_event::<StatueUnlocked>()
        .add_event::<StatueUnlockedDelayed>();
    }
}