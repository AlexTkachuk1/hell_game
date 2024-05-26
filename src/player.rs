use bevy::{math::vec3, prelude::*};
use crate::state::GameState;
use crate::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut transform, mut player_state) = player_query.single_mut();

    let w_key: bool =
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let s_key: bool =
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let a_key: bool =
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let d_key: bool =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }

    if delta.is_finite()
        && (delta.x.abs() > 0. || delta.y.abs() > 0.)
        && (w_key || s_key || a_key || d_key)
    {
        transform.translation += vec3(delta.x, delta.y, 0.).normalize() * PLAYER_SPEED;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}
