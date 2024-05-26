use self::{
    gun::{Gun, GunTimer},
    player::Player,
    state::GameState,
};
use crate::*;
use animation::AnimationTimer;
use bevy::{math::vec3, prelude::*, time::Stopwatch};
use player::{Health, PlayerState};
use rand::Rng;

#[derive(Component)]
pub struct GameEntity;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorrations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
        Health(PLAYER_HEALTH),
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        GameEntity,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 17,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Gun,
        GunTimer(Stopwatch::new()),
        GameEntity,
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decorrations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_DECORRATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        commands.spawn((
            SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
            transform: Transform::from_translation(vec3(x, y, -1.))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
