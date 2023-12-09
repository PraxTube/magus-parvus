use bevy::math::vec2;
use bevy::prelude::*;

use crate::{
    player::Player,
    ui::world_text::{SpawnWorldText, WorldText},
    GameState,
};

use super::{Spell, SpellCasted};

fn spawn_deaths(
    q_player: Query<&Transform, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
    mut ev_spawn_world_text: EventWriter<SpawnWorldText>,
) {
    let pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Flub {
            ev_spawn_world_text.send(SpawnWorldText {
                world_text: WorldText {
                    offset: Vec3::new(0.0, 20.0, 10.0),
                    ..default()
                },
                pos,
                content: "FLUB".to_string(),
            });

            let time = 4.0;
            let scale_points = [[
                vec2(0.0, 1.0),
                vec2(0.14, 1.0),
                vec2(0.533, 1.0),
                vec2(1.0, 1.0),
            ]];
            let alpha_points = [[
                vec2(0.0, 0.0),
                vec2(0.15, 2.0),
                vec2(0.533, 0.507),
                vec2(1.0, 0.0),
            ]];
            let scale_curve = CubicBezier::new(scale_points).to_curve();
            let alpha_curve = CubicBezier::new(alpha_points).to_curve();

            ev_spawn_world_text.send(SpawnWorldText {
                world_text: WorldText {
                    offset: Vec3::new(0.0, -10.0, 10.0),
                    font_scale: 8.0,
                    timer: Timer::from_seconds(time, TimerMode::Once),
                    scale_curve,
                    alpha_curve,
                    ..default()
                },
                pos,
                content: "Did you mean 'fireball'?".to_string(),
            });
        }
    }
}

pub struct FlubPlugin;

impl Plugin for FlubPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_deaths,).run_if(in_state(GameState::Gaming)));
    }
}
