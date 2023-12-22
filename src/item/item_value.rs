use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{spell::Spell, GameAssets};

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
        Item::IgnisPila => "Cast 5 fireballs.",
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
        Item::Fulgur => 150.0,
        Item::IgnisPila => 150.0,
        Item::AerTracto => 200.0,
        Item::InfernoPila => 200.0,
        Item::ScutumGlaciei => 225.0,
        Item::AerPello => 300.0,
        Item::FulgurAvis => 300.0,
    }
}

pub fn item_spell(item: &Item) -> Spell {
    match item {
        Item::NotImplemented => Spell::Flub,
        Item::Tutorial => Spell::Fireball,
        Item::IgnisPila => Spell::IgnisPila,
        Item::InfernoPila => Spell::InfernoPila,
        Item::Fulgur => Spell::Fulgur,
        Item::ScutumGlaciei => Spell::ScutumGlaciei,
        Item::AerTracto => Spell::AerTracto,
        Item::AerPello => Spell::AerPello,
        Item::FulgurAvis => Spell::FulgurAvis,
    }
}

pub fn statue_sub_spawner(statue: &Statue) -> Vec<(f32, EnemySubSpawner)> {
    match statue.item {
        Item::NotImplemented => Vec::new(),
        Item::Tutorial => Vec::new(),
        Item::Fulgur => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 1,
                angle: PI / 2.0,
                radius: 100.0,
                spawn_formation: SpawnFormation::Group,
                ..default()
            },
        )],
        Item::IgnisPila => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 3,
                angle: 0.0,
                radius: 100.0,
                spawn_formation: SpawnFormation::Group,
                ..default()
            },
        )],
        Item::AerTracto => vec![
            (
                3.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 3,
                    spawn_formation: SpawnFormation::Group,
                    angle: 3.0 / 2.0 * PI,
                    timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 3,
                    spawn_formation: SpawnFormation::Circle,
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
        Item::InfernoPila => vec![(
            0.0,
            EnemySubSpawner {
                statue: statue.clone(),
                count: 6,
                radius: 160.0,
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                spawn_formation: SpawnFormation::Circle,
                ..default()
            },
        )],
        Item::ScutumGlaciei => vec![
            (
                5.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 6,
                    radius: 175.0,
                    spawn_formation: SpawnFormation::Circle,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 8,
                    radius: 210.0,
                    spawn_formation: SpawnFormation::Random,
                    timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
        Item::AerPello => vec![
            (
                8.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 8,
                    radius: 210.0,
                    spawn_formation: SpawnFormation::Circle,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 15,
                    radius: 240.0,
                    angle: 0.0,
                    spawn_formation: SpawnFormation::Group,
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 15,
                    radius: 240.0,
                    angle: PI,
                    spawn_formation: SpawnFormation::Group,
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
        Item::FulgurAvis => vec![
            (
                10.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 10,
                    spawn_formation: SpawnFormation::Circle,
                    radius: 150.0,
                    timer: Timer::from_seconds(0.0, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                10.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 15,
                    spawn_formation: SpawnFormation::Random,
                    radius: 220.0,
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 50,
                    spawn_formation: SpawnFormation::Circle,
                    radius: 240.0,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    ..default()
                },
            ),
        ],
    }
}
