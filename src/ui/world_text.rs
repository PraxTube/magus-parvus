use bevy::math::{cubic_splines::CubicCurve, vec2};
use bevy::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState};

// This number will change the sharpness of the font.
// The higher it is, the sharper the text.
const FONT_SCALE_RATIO: f32 = 100.0;

#[derive(Component, Clone)]
pub struct WorldText {
    pub font_scale: f32,
    pub font_color: Color,
    pub offset: Vec3,
    pub random_spray_intensity: f32,
    pub timer: Timer,
    pub scale_curve: CubicCurve<Vec2>,
    pub alpha_curve: CubicCurve<Vec2>,
}

#[derive(Event)]
pub struct SpawnWorldText {
    pub world_text: WorldText,
    pub pos: Vec3,
    pub content: String,
}

impl Default for WorldText {
    fn default() -> Self {
        let scale_points = [[
            vec2(0.0, 0.5),
            vec2(0.14, 3.035),
            vec2(0.533, 0.507),
            vec2(1.0, 1.0),
        ]];
        let alpha_points = [[
            vec2(0.0, 0.5),
            vec2(0.14, 3.035),
            vec2(0.533, 0.507),
            vec2(1.0, 0.0),
        ]];
        let scale_curve = CubicBezier::new(scale_points).to_curve().unwrap();
        let alpha_curve = CubicBezier::new(alpha_points).to_curve().unwrap();
        Self {
            font_scale: 10.0,
            font_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
            offset: Vec3::new(0.0, 10.0, 10.0),
            random_spray_intensity: 5.0,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            scale_curve,
            alpha_curve,
        }
    }
}

fn spawn_world_text(commands: &mut Commands, assets: &Res<GameAssets>, ev: &SpawnWorldText) {
    let text_style = TextFont {
        font: assets.font.clone(),
        font_size: FONT_SCALE_RATIO,
        ..default()
    };

    let mut rng = rand::thread_rng();
    let rand_offset = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
        * ev.world_text.random_spray_intensity;

    commands.spawn((
        ev.world_text.clone(),
        Text2d::from(ev.content.to_string()),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(ev.world_text.font_color),
        text_style,
        Transform::from_translation(ev.pos + rand_offset + ev.world_text.offset)
            .with_scale(Vec3::splat(ev.world_text.font_scale / FONT_SCALE_RATIO)),
    ));
}

fn spawn_world_texts(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_world_text: EventReader<SpawnWorldText>,
) {
    for ev in ev_spawn_world_text.read() {
        spawn_world_text(&mut commands, &assets, ev);
    }
}

fn despawn_world_texts(mut commands: Commands, q_world_texts: Query<(Entity, &WorldText)>) {
    for (entity, world_text) in &q_world_texts {
        if world_text.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_world_texts(
    time: Res<Time>,
    mut q_world_texts: Query<(&mut Transform, &mut TextColor, &mut WorldText)>,
) {
    for (mut transform, mut text_color, mut world_text) in &mut q_world_texts {
        world_text.timer.tick(time.delta());

        let duration = world_text.timer.duration().as_secs_f32();
        let t = world_text.timer.elapsed_secs() / duration;

        transform.scale = world_text.scale_curve.position(t).y
            * Vec3::splat(world_text.font_scale / FONT_SCALE_RATIO);
        let new_alpha = world_text.alpha_curve.position(t).y;
        text_color.0 = world_text.font_color.with_alpha(new_alpha);
    }
}

pub struct WorldTextPlugin;

impl Plugin for WorldTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_world_texts).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(Update, (despawn_world_texts, animate_world_texts))
        .add_event::<SpawnWorldText>();
    }
}
