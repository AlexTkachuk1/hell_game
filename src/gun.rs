use crate::*;
use crate::{player::Player, state::GameState, CursorPosition};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::Stopwatch,
};
use std::f32::consts::PI;
use std::time::Instant;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct SpawnInstant(Instant);

#[derive(Component)]
pub struct BulletDirection(Vec3);

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                  update_gun_transform,
                  handle_gun_input,
                  update_bullets,
                  despawn_old_bullets,
            ).run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            commands.entity(e).despawn();
        }
    }
}

fn update_gun_transform(
    cursour_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if gun_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos: Vec2 = match cursour_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 30.0,
    );

    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 15.0;
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
        t.translation.z = 10.0;
    }
}

fn handle_gun_input(
    mut commands: Commands,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    time: Res<Time>,
    handle: Res<GlobalTextureAtlas>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    
    let mut player = player_query.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) {
        player.attacks = true;
    }

    if !mouse_button_input.pressed(MouseButton::Left) || gun_query.is_empty() {
        player.attacks = false;
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    let bullet_direction = gun_transform.local_x();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 16,
                  },
                transform: Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Bullet,
            BulletDirection(*bullet_direction),
            SpawnInstant(Instant::now()),
        ));
    }
}
