use std::f32::consts::TAU;

use rand::{thread_rng, Rng};

use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    player::Player,
    ui::health::Health,
    utils::anim_sprite::{AnimSprite, AnimSpriteTimer},
    world::camera::YSort,
    GameAssets, GameState,
};

use super::{Spell, SpellCasted};

const DAMAGE: f32 = 999.0;
const SCALE: f32 = 2.0;
const DISTANCE: f32 = 100.0;
const SPRITES: usize = 18;
const SPRITE_TIME: f32 = 0.075;

const START_ALBEDO: f32 = 0.5;
const SCREEN_COLOR: Color = Color::rgba(0.4, 0.2, 0.3, START_ALBEDO);

#[derive(Component)]
struct Death {
    timer: Timer,
    timeout: bool,
}

#[derive(Component)]
struct ScreenEffect {
    timer: Timer,
    disabled: bool,
}

#[derive(Event)]
struct KillAll;

impl Default for ScreenEffect {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
            disabled: false,
        }
    }
}

impl Default for Death {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SPRITES as f32 * SPRITE_TIME, TimerMode::Once),
            timeout: false,
        }
    }
}

fn spawn_deaths(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    let mut rng = thread_rng();
    let offset = Quat::from_rotation_z(rng.gen_range(0.0..TAU)).mul_vec3(Vec3::X) * DISTANCE;

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Death {
            commands.spawn((
                Death::default(),
                YSort(100.0),
                AnimSprite::new(SPRITES, true),
                AnimSpriteTimer::new(SPRITE_TIME),
                SpriteSheetBundle {
                    transform: Transform::from_translation(pos + offset)
                        .with_scale(Vec3::splat(SCALE)),
                    texture_atlas: assets.death.clone(),
                    ..default()
                },
            ));
        }
    }
}

fn tick_death_timer(time: Res<Time>, mut q_deaths: Query<&mut Death>) {
    for mut death in &mut q_deaths {
        death.timer.tick(time.delta());

        if death.timer.just_finished() {
            death.timeout = true;
        }
    }
}

fn tick_screen_effect_timers(time: Res<Time>, mut q_screen_effects: Query<&mut ScreenEffect>) {
    for mut screen_effect in &mut q_screen_effects {
        screen_effect.timer.tick(time.delta());

        if screen_effect.timer.just_finished() {
            screen_effect.disabled = true;
        }
    }
}

fn animate_screen_effects(mut q_screen_effects: Query<(&ScreenEffect, &mut BackgroundColor)>) {
    for (screen_effect, mut color) in &mut q_screen_effects {
        let time =
            screen_effect.timer.elapsed_secs() / screen_effect.timer.duration().as_secs_f32();
        *color = BackgroundColor(Color::rgba(
            SCREEN_COLOR.r(),
            SCREEN_COLOR.g(),
            SCREEN_COLOR.b(),
            (1.0 - time) * START_ALBEDO,
        ));
    }
}

fn despawn_deaths(
    mut commands: Commands,
    q_deaths: Query<(Entity, &Death)>,
    mut ev_kill_all: EventWriter<KillAll>,
) {
    for (entity, death) in &q_deaths {
        if !death.timeout {
            continue;
        }

        ev_kill_all.send(KillAll);
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_screen_effects(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_kill_all: EventReader<KillAll>,
) {
    for _ in ev_kill_all.read() {
        commands.spawn((
            ScreenEffect::default(),
            ImageBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage {
                    texture: assets.white_pixel.clone(),
                    ..default()
                },
                background_color: BackgroundColor(SCREEN_COLOR),
                ..default()
            },
        ));
    }
}

fn despawn_screen_effects(
    mut commands: Commands,
    q_screen_effects: Query<(Entity, &ScreenEffect)>,
) {
    for (entity, screen_effect) in &q_screen_effects {
        if screen_effect.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn kill_all_enemies(
    mut q_enemies: Query<&mut Health, With<Enemy>>,
    mut ev_kill_all: EventReader<KillAll>,
) {
    for _ in ev_kill_all.read() {
        for mut health in &mut q_enemies {
            health.health -= DAMAGE;
        }
    }
}

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_deaths,
                tick_death_timer,
                tick_screen_effect_timers,
                animate_screen_effects,
                despawn_deaths,
                spawn_screen_effects,
                despawn_screen_effects,
                kill_all_enemies,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<KillAll>();
    }
}
