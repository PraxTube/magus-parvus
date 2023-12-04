use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::Rng;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

const SCALE: f32 = 3.0;
const FULGUR_COUNT: usize = 100;

const SPRITES_COUNT: usize = 12;
const SPRITES_TIME: f32 = 0.05;
const SPRITE_HEIGHT_HALF: f32 = 48.0;

const POSITION_OFFSET: Vec3 = Vec3::new(0.0, SPRITE_HEIGHT_HALF * SCALE, 0.0);
const RAND_OFFSET: f32 = 10.0;

#[derive(Component)]
pub struct Lightning {
    pub damage: f32,
}

impl Default for Lightning {
    fn default() -> Self {
        Self { damage: 100.0 }
    }
}

fn spawn_lightning(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let entity = commands
        .spawn((
            Lightning::default(),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.lightning.clone(),
                ..default()
            },
            AnimSprite::new(SPRITES_COUNT, false),
            AnimSpriteTimer::new(SPRITES_TIME),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(5.0),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0,
                -SPRITE_HEIGHT_HALF,
                0.0,
            ))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}

fn spawn_fulgur(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_enemies: Query<&Transform, With<Enemy>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    fn get_pos(rng: &mut ThreadRng, t: &Transform) -> Transform {
        Transform::from_translation(
            t.translation
                + POSITION_OFFSET
                + Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0) * RAND_OFFSET,
        )
        .with_scale(Vec3::splat(SCALE))
    }

    for ev in ev_spell_casted.read() {
        // The sum of the for loops will always equal `FULGUR_COUNT`.
        // So the runtime of this is exactly `O(FULGUR_COUNT)`.
        // The only exception is if there are no enemies to target.
        if ev.spell == Spell::Fulgur {
            let mut rng = rand::thread_rng();
            let k = q_enemies.iter().count();
            if k == 0 {
                continue;
            }

            let m = FULGUR_COUNT / k;
            let n = FULGUR_COUNT % k;

            for _ in 0..m {
                for transform in q_enemies.iter() {
                    spawn_lightning(&mut commands, &assets, get_pos(&mut rng, transform));
                }
            }

            for transform in q_enemies.iter().choose_multiple(&mut rng, n) {
                spawn_lightning(&mut commands, &assets, get_pos(&mut rng, transform));
            }
        }
    }
}

pub struct LightningPlugin;

impl Plugin for LightningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_fulgur).run_if(in_state(GameState::Gaming)));
    }
}
