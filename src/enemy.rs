use std::f32::consts::PI;
use std::time::Duration;

use crate::player::Player;
use crate::state::GameState;
use crate::*;
use animation::AnimationTimer;
use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use castle::Castle;
use gold::Gold;
use rand::Rng;
use world::GameEntity;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

#[derive(Component, PartialEq, Eq)]
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

pub struct EnemyPlagin;

impl Plugin for EnemyPlagin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity, &Transform), With<Enemy>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity, transform) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();

            commands.spawn((
                SpriteSheetBundle {
                    texture: handle.coin_image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.coin_layout.clone().unwrap(),
                        index: 0,
                    },
                    transform: Transform::from_translation(vec3(
                        transform.translation.x,
                        transform.translation.y,
                        -1.,
                    ))
                    .with_scale(Vec3::splat(COIN_SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                Gold,
                AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
                GameEntity,
            ));
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    castle_query: Query<&Transform, With<Castle>>,
    mut enemy_query: Query<
        (&mut Transform, &EnemyType),
        (With<Enemy>, Without<Castle>, Without<Player>),
    >,
) {
    if enemy_query.is_empty() || castle_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    let castle_pos = castle_query.single().translation;
    for (mut transform, enemy_type) in enemy_query.iter_mut() {
        let mut dir = (castle_pos - transform.translation).normalize();
        
        if enemy_type == &EnemyType::Green {
            dir = (player_pos - transform.translation).normalize();
        } 

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
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

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
                transform: Transform::from_translation(vec3(x, y, 1.))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Enemy::default(),
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            GameEntity,
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

    return (random_x, random_y);
}
