use bevy::prelude::*;

use crate::{
    castle::Castle, enemy::{Enemy, EnemyType}, gold::Gold, gui::MenuBG, gun::Gun, player::{Player, PlayerState}, CursorPosition
};
use crate::state::GameState;

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
                animation_timer_tick,
                animate_menu_bg
            )
            .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                animate_enemy,
                animate_gold,
                animate_castle,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
            .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_menu_bg(
    mut menu_query: Query<(&mut TextureAtlas, &AnimationTimer), With<MenuBG>>,
) {
    if menu_query.is_empty() {
        return;
    }

    let (mut atlas, timer) = menu_query.single_mut();

    if timer.just_finished() {
        atlas.index = (atlas.index + 1) % 250;
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut atlas, state, timer) = player_query.single_mut();
    if timer.just_finished() {
        let base_sprite_index = match state {
            PlayerState::Idle => 0,
            PlayerState::Run => 12,
        };
        atlas.index = base_sprite_index + (atlas.index + 1) % 12;
    }
}

fn animate_gold(
    mut gold_query: Query<(&mut TextureAtlas, &AnimationTimer), With<Gold>>,
) {
    if gold_query.is_empty() {
        return;
    }

    for (mut atlas, timer) in gold_query.iter_mut() {
        if timer.just_finished() {
            atlas.index = 0 + (atlas.index + 1) % 6;
        }
    }
}

fn animate_castle(
    mut castle_query: Query<(&mut TextureAtlas, &AnimationTimer), With<Castle>>,
) {
    if castle_query.is_empty() {
        return;
    }

    for (mut atlas, timer) in castle_query.iter_mut() {
        if timer.just_finished() {
            atlas.index = (atlas.index + 1) % 28;
        }
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut TextureAtlas, &AnimationTimer, &EnemyType), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut atlas, timer, enemy_type) in enemy_query.iter_mut() {
        if timer.just_finished() {
            atlas.index = enemy_type.get_base_sprite_index() + (atlas.index + 1) % 4;
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}
