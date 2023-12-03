use rand::seq::IteratorRandom;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

#[derive(Component)]
pub struct Lightning {
    pub damage: f32,
}

impl Default for Lightning {
    fn default() -> Self {
        Self { damage: 1.0 }
    }
}

const SCALE: f32 = 3.0;
const FULGUR_COUNT: usize = 4;
const SPRITES_COUNT: usize = 12;
const SPRITES_TIME: f32 = 0.05;
const SPRITE_HEIGHT_HALF: f32 = 48.0;
const POSITION_OFFSET: Vec3 = Vec3::new(0.0, SPRITE_HEIGHT_HALF * SCALE, 0.0);

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
    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Fulgur {
            let mut rng = rand::thread_rng();
            let transforms = q_enemies.iter().choose_multiple(&mut rng, FULGUR_COUNT);

            for transform in transforms {
                let transform =
                    Transform::from_translation(transform.translation + POSITION_OFFSET)
                        .with_scale(Vec3::splat(SCALE));
                spawn_lightning(&mut commands, &assets, transform);
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
