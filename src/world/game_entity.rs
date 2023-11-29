use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ui::health::Health;

#[derive(Event, Clone)]
pub struct SpawnGameEntity {
    pub entity: Entity,
    pub health: Health,
}

pub struct GameEntityPlugin;

impl Plugin for GameEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnGameEntity>()
            .add_systems(Update, display_events);
    }
}

pub fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {collision_event:?}");
    }
}
