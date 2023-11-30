use bevy::prelude::*;

use crate::ui::health::Health;

#[derive(Event, Clone)]
pub struct SpawnGameEntity {
    pub entity: Entity,
    pub health: Health,
}

pub struct GameEntityPlugin;

impl Plugin for GameEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnGameEntity>();
    }
}
