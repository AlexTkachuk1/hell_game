use bevy::prelude::*;

use crate::{
    animation::AnimationTimer, player::Health, state::GameState, world::GameEntity,
    GlobalTextureAtlas, CASTLE_HEALTH, CASTLE_SPRITE_SCALE_FACTOR, ENEMY_DAMAGE,
};

#[derive(Event)]
pub struct CastleEnemyCollisionEvent;

#[derive(Component)]
pub struct Castle;
pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastleEnemyCollisionEvent>()
            .add_systems(OnEnter(GameState::InGame), spawn_castle)
            .add_systems(
                Update,
                (handle_castle_enemy_collision_events, handle_castle_death)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn handle_castle_death(
    castle_query: Query<&Health, With<Castle>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if castle_query.is_empty() {
        return;
    }

    let health = castle_query.single();

    if health.0 <= 0.0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_castle_enemy_collision_events(
    mut castle_query: Query<&mut Health, With<Castle>>,
    mut events: EventReader<CastleEnemyCollisionEvent>,
) {
    if castle_query.is_empty() {
        return;
    }

    let mut health = castle_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
}

fn spawn_castle(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.castle_image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.castle_layout.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(CASTLE_SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Castle,
        Health(CASTLE_HEALTH),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        GameEntity,
    ));
}
