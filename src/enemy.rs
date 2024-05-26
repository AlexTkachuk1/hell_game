use std::f32::consts::PI;
use std::time::Duration;

use animation::AnimationTimer;
use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use crate::player::Player;
use crate::*;
use crate::state::GameState;

pub struct EnemyPlagin;

#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

impl EnemyType {
      fn get_rand_enemy() -> Self {
          let mut rng = rand::thread_rng();
          let rand_index = rng.gen_range(0..3);
          return match rand_index {
              0 => Self::Green,
              1 => Self::Red,
              _ => Self::Skin,
          };
      }
  
      pub fn get_base_sprite_index(&self) -> usize {
          match self {
              EnemyType::Green => 8,
              EnemyType::Red => 12,
              EnemyType::Skin => 20,
          }
      }
}

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

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
      let (x, y) = get_random_position_around(player_pos);
      let enemy_type = EnemyType::get_rand_enemy();

      commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: enemy_type.get_base_sprite_index(),
                },
                transform: Transform::from_translation(vec3(x, y, 1.)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Enemy,
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
      ));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
      let mut rng = rand::thread_rng();
      let angle = rng.gen_range(0.0..PI * 2.0);
      let dist = rng.gen_range(1000.0..5000.0);
  
      let offset_x = angle.cos() * dist;
      let offset_y = angle.sin() * dist;
  
      let random_x = pos.x + offset_x;
      let random_y = pos.y + offset_y;
  
      (random_x, random_y)
  }