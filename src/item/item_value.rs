use std::f32::consts::PI;

use bevy::prelude::*;

use crate::GameAssets;

use super::{
    enemy_sub_spawner::{EnemySubSpawner, SpawnFormation},
    statue::Statue,
    Item,
};

pub fn item_title(item: &Item) -> String {
    match item {
        Item::NotImplemented => "NOT IMPLEMENTED, you should not see this, please report",
        Item::Tutorial => "Spell Console",
        Item::IgnisPila => "Ignis Pila",
        Item::InfernoPila => "Inferno Pila",
        Item::Fulgur => "Fulgur",
        Item::ScutumGlaciei => "Scutum Glaciei",
        Item::AerTracto => "Aer Tracto",
        Item::AerPello => "Aer Pello",
        Item::FulgurAvis => "Fulgur Avis",
    }
    .to_string()
}

pub fn item_description(item: &Item) -> String {
    match item {
        Item::NotImplemented => "CONTENT DESCRIPTION",
        Item::Tutorial => {
            "Press 'i' to open your spell console.\nThen type your spell, try 'fireball'."
        }
        Item::IgnisPila => "Cast 8 fireballs omni directionally.",
        Item::InfernoPila => "Cast MANY fireballs omni directionally",
        Item::Fulgur => {
            "Call down lightning strikes on random enemies.\nOnly works when there are enemies."
        }
        Item::ScutumGlaciei => "Materialize 10 ice crystals that cycle around you for 10 seconds.",
        Item::AerTracto => "Pull enemies towards you.",
        Item::AerPello => "Push enemies away from you.",
        Item::FulgurAvis => "Summon a powerful lightning bird.",
    }
    .to_string()
}

pub fn item_icon(assets: &Res<GameAssets>, item: &Item) -> Handle<Image> {
    match item {
        Item::NotImplemented => assets.placeholder_icon.clone(),
        Item::Tutorial => assets.spell_console_icon.clone(),
        Item::IgnisPila => assets.ignis_pila_icon.clone(),
        Item::InfernoPila => assets.inferno_pila_icon.clone(),
        Item::Fulgur => assets.fulgur_icon.clone(),
        Item::ScutumGlaciei => assets.scutum_glaciei_icon.clone(),
        Item::AerTracto => assets.aer_tracto_icon.clone(),
        Item::AerPello => assets.aer_pello_icon.clone(),
        Item::FulgurAvis => assets.fulgur_avis_icon.clone(),
    }
}

pub fn item_wall_offset(item: &Item) -> f32 {
    match item {
        Item::NotImplemented => 0.0,
        Item::Tutorial => 0.0,
        Item::IgnisPila => 200.0,
        Item::InfernoPila => 300.0,
        Item::Fulgur => 300.0,
        Item::ScutumGlaciei => 300.0,
        Item::AerTracto => 300.0,
        Item::AerPello => 300.0,
        Item::FulgurAvis => 300.0,
    }
}

pub fn statue_sub_spawner(statue: &Statue) -> Vec<(f32, EnemySubSpawner)> {
    match statue.item {
        Item::NotImplemented => Vec::new(),
        Item::Tutorial => Vec::new(),
        Item::IgnisPila => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 1,
                angle: PI / 2.0,
                spawn_formation: SpawnFormation::Group,
                ..default()
            },
        )],
        Item::InfernoPila => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 6,
                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                spawn_formation: SpawnFormation::Circle,
                ..default()
            },
        )],
        Item::Fulgur => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 2,
                spawn_formation: SpawnFormation::Random,
                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                ..default()
            },
        )],
        Item::ScutumGlaciei => vec![
            (
                5.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 5,
                    spawn_formation: SpawnFormation::Circle,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 5,
                    spawn_formation: SpawnFormation::Group,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
        Item::AerTracto => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 10,
                spawn_formation: SpawnFormation::Random,
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                ..default()
            },
        )],
        Item::AerPello => vec![(
            1.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 15,
                spawn_formation: SpawnFormation::Group,
                timer: Timer::from_seconds(0.0, TimerMode::Repeating),
                ..default()
            },
        )],
        Item::FulgurAvis => vec![
            (
                5.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 10,
                    spawn_formation: SpawnFormation::Circle,
                    offset: 100.0,
                    timer: Timer::from_seconds(0.0, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 20,
                    spawn_formation: SpawnFormation::Random,
                    offset: 200.0,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
    }
}
