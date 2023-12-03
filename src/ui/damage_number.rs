use bevy::math::{cubic_splines::CubicCurve, vec2};
use bevy::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState};

const TIME: f32 = 1.0;
const SCALE: f32 = 7.5;
// This number will change the sharpness of the font.
// The higher it is, the sharper the text.
const FONT_SCALE_RATIO: f32 = 100.0;
const OFFSET: Vec3 = Vec3::new(0.0, 10.0, 10.0);
const RANDOM_SPRAY: f32 = 5.0;

#[derive(Component)]
pub struct DamageText {
    timer: Timer,
    scale_curve: CubicCurve<Vec2>,
    alpha_curve: CubicCurve<Vec2>,
}

impl Default for DamageText {
    fn default() -> Self {
        let scale_points = [[
            vec2(0.0, 0.5),
            vec2(0.14, 3.035),
            vec2(0.533, 0.507),
            vec2(1.0, 1.0),
        ]];
        let color_points = [[
            vec2(0.0, 0.5),
            vec2(0.14, 3.035),
            vec2(0.533, 0.507),
            vec2(1.0, 0.0),
        ]];
        let scale_curve = CubicBezier::new(scale_points).to_curve();
        let color_curve = CubicBezier::new(color_points).to_curve();
        Self {
            timer: Timer::from_seconds(TIME, TimerMode::Once),
            scale_curve,
            alpha_curve: color_curve,
        }
    }
}

#[derive(Event)]
pub struct SpawnDamageText {
    pub pos: Vec3,
    pub damage: u32,
}

fn spawn_damage_text(commands: &mut Commands, assets: &Res<GameAssets>, ev: &SpawnDamageText) {
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: FONT_SCALE_RATIO,
        color: Color::rgba(1.0, 1.0, 1.0, 0.0),
    };

    let mut rng = rand::thread_rng();
    let rand_offset =
        Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0) * RANDOM_SPRAY;

    commands.spawn((
        DamageText::default(),
        Text2dBundle {
            text: Text::from_section(ev.damage.to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(ev.pos + rand_offset + OFFSET)
                .with_scale(Vec3::splat(SCALE / FONT_SCALE_RATIO)),
            ..default()
        },
    ));
}

fn spawn_damage_texts(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_damage_text: EventReader<SpawnDamageText>,
) {
    for ev in ev_spawn_damage_text.read() {
        spawn_damage_text(&mut commands, &assets, ev);
    }
}

fn despawn_damage_texts(mut commands: Commands, q_damage_texts: Query<(Entity, &DamageText)>) {
    for (entity, damage_text) in &q_damage_texts {
        if damage_text.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn animate_damage_texts(
    time: Res<Time>,
    mut q_damage_texts: Query<(&mut Transform, &mut Text, &mut DamageText)>,
) {
    for (mut transform, mut text, mut damage_text) in &mut q_damage_texts {
        damage_text.timer.tick(time.delta());

        let duration = damage_text.timer.duration().as_secs_f32();
        let t = damage_text.timer.elapsed_secs() / duration;

        transform.scale =
            damage_text.scale_curve.position(t).y * Vec3::splat(SCALE / FONT_SCALE_RATIO);
        text.sections[0].style.color =
            Color::rgba(1.0, 1.0, 1.0, damage_text.alpha_curve.position(t).y);
    }
}

pub struct DamageNumberPlugin;

impl Plugin for DamageNumberPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDamageText>().add_systems(
            Update,
            (
                spawn_damage_texts,
                despawn_damage_texts,
                animate_damage_texts,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
