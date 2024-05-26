use std::time::Duration;

use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use crate::player::Player;
use crate::*;
use crate::state::GameState;

pub struct EnemyPlagin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlagin {
      fn build(&self, app: &mut App) {
            app.add_systems(
                  Update,
                  (spawn_enemies
                        .run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))), update_enemy_transform)
                  .run_if(in_state(GameState::InGame)));
      }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
      if enemy_query.is_empty() || player_query.is_empty() {
          return;
      }

      let player_pos = player_query.single().translation;
      for mut transform in enemy_query.iter_mut() {
            let dir = (player_pos - transform.translation).normalize();
            transform.translation += dir * ENEMY_SPEED; 

      }
}

fn spawn_enemies(
      mut commands: Commands,
      handle: Res<GlobalTextureAtlas>,
      player_query: Query<&Transform, With<Player>>,
      enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(10);

    if enemy_spawn_count >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    for _ in 0..enemy_spawn_count {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(-WORLD_W..WORLD_W);
      let y = rng.gen_range(-WORLD_H..WORLD_H);

      commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 4,
                },
                transform: Transform::from_translation(vec3(x, y, 1.)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Enemy,
      ));
    }
}
