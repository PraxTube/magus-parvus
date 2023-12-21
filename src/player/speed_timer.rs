use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Default)]
pub struct SpeedTimer {
    pub elapsed: f32,
}

fn tick(time: Res<Time>, mut speed_timer: ResMut<SpeedTimer>) {
    speed_timer.elapsed += time.delta_seconds();
}

pub struct SpeedTimerPlugin;

impl Plugin for SpeedTimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeedTimer>()
            .add_systems(Update, (tick,).run_if(in_state(GameState::Gaming)));
    }
}
