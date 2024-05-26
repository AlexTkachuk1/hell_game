use bevy::ecs::entity;
use bevy::utils::tracing::Instrument;
use bevy::utils::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::player::{Player, PlayerEnemyCollisionEvent};
use crate::*;
use crate::{enemy::Enemy, gun::Bullet, state::GameState};

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}

impl KdPoint for Collidable {
      type Scalar = f32;
      type Dim = typenum::U2;
      fn at(&self, k: usize) -> f32 {
          if k == 0 {
              return self.pos.x;
          }
  
          self.pos.y
      }
  }  

#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Default for EnemyKdTree {
      fn default() -> Self {
          Self(KdTree::build_by_ordered_float(vec![]))
      }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
            //     handle_enemy_player_collision,
                update_enemy_kd_tree
                    .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (t, e) in enemy_query.iter() {
        items.push(Collidable {
            entity: e,
            pos: t.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (b_t, entity) in bullet_query.iter() {
        let pos = b_t.translation;
        let enemies = tree.0.within_radius(&[pos.x, pos.y], 25.0);

        for e in enemies {
            if let Ok((_, mut enemy)) = enemy_query.get_mut(e.entity) {
                  enemy.health -= BULLET_DAMAGE;
                  commands.entity(entity).despawn();
                  return;
            }
        }
    }
}