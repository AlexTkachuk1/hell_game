use bevy::{math::vec3, prelude::*};

use crate::pan_cam::{PanCam, PanCamPlugin};
use crate::player::Player;
use crate::state::GameState;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 1.5;

    commands.spawn(camera_bundle).insert(PanCam {
        grab_buttons: vec![MouseButton::Right],
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.5,
        max_scale: Some(2.5),
        ..default()
    });
}



fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        let mut camera_transform = camera_query.single_mut();
        let player_transform = player_query.single().translation;
        let (x, y) = (player_transform.x, player_transform.y);
    
        camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.01);
    }
}
