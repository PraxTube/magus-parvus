use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::cast::{DemonSpell, SpawnDemonSpell};

fn spawn_explosion_spells(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spawn_demon_spells: EventReader<SpawnDemonSpell>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for ev in ev_spawn_demon_spells.read() {
        if ev.spell != DemonSpell::Explosion {
            continue;
        }

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.demon_boss_explosion_animations[0].clone());

        commands.spawn((
            animator,
            YSort(1.0),
            SpriteSheetBundle {
                texture_atlas: assets.demon_boss_explosion.clone(),
                transform: Transform::from_translation(player_pos).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ));
    }
}

pub struct DemonBossExplosionPlugin;

impl Plugin for DemonBossExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_explosion_spells,).run_if(in_state(GameState::Gaming)),
        );
    }
}
