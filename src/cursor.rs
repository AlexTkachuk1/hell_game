use bevy::prelude::*;

use crate::{
    state::GameState, world::GameEntity, GlobalTextureAtlas, CURSOR_SPRITE_SCALE_FACTOR
};

pub struct CursorPlugin;

#[derive(Component)]
struct GameCursor;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_cursor)
            .add_systems(OnEnter(GameState::InGame), set_gun_cursor)
            .add_systems(Update, (move_cursor, on_click_cursor));
    }
}

fn setup_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
) {
        let mut window: Mut<Window> = windows.single_mut();
        window.cursor.visible = false;
    
    commands.spawn((
        AtlasImageBundle {
            style: Style {
                width: Val::Px(40.),
                height: Val::Px(40.),
                position_type: PositionType::Absolute,
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: handle.cursor_layout.clone().unwrap(),
                index: 1,
            },
            image: UiImage::new(handle.cursor_image.clone().unwrap()),
            ..default()
        },
        GameCursor,
        GameEntity,
    ));
}

fn move_cursor(
    window: Query<&mut Window>,
    mut cursor_query: Query<&mut Style, With<GameCursor>>,
) {
    if cursor_query.is_empty() {
        return;
    }

    let windows: &Window = window.single();
    if let Some(position) = windows.cursor_position() {
            let mut style = cursor_query.single_mut();
            style.left = Val::Px(position.x);
            style.top = Val::Px(position.y);
    }
}

fn set_gun_cursor(mut cursor_query: Query<&mut TextureAtlas, With<GameCursor>>) {
    if cursor_query.is_empty() {
        return;
    }

    let mut atlas = cursor_query.single_mut();
    atlas.index = 0;
}

// fn set_default_cursor(
//       mut cursor_query: Query<&mut TextureAtlas, With<GameCursor>>
// ) {
//     if cursor_query.is_empty() {
//         return;
//     }

//     let mut atlas = cursor_query.single_mut();
//     atlas.index = 1;
// }

// fn set_hand_cursor(
//       mut cursor_query: Query<&mut TextureAtlas, With<GameCursor>>
// ) {
//     if cursor_query.is_empty() {
//         return;
//     }

//     let mut atlas = cursor_query.single_mut();
//     atlas.index = 2;
// }

// fn set_destroy_cursor(
//       mut cursor_query: Query<&mut TextureAtlas, With<GameCursor>>
// ) {
//     if cursor_query.is_empty() {
//         return;
//     }

//     let mut atlas = cursor_query.single_mut();
//     atlas.index = 3;
// }

fn on_click_cursor(
      mouse_button_input: Res<ButtonInput<MouseButton>>,
      mut cursor_query: Query<(&mut Transform, &mut TextureAtlas), With<GameCursor>>,
      state: Res<State<GameState>>,
) {
      if cursor_query.is_empty() {
            return;
      }

      let (mut transform, mut atlas) = cursor_query.single_mut();

      if mouse_button_input.pressed(MouseButton::Left) {
            transform.scale = Vec3::splat(CURSOR_SPRITE_SCALE_FACTOR - 0.1);
      } else if mouse_button_input.pressed(MouseButton::Right) {
            atlas.index = 2;
      } else {
            transform.scale = Vec3::splat(CURSOR_SPRITE_SCALE_FACTOR);

            if *state.get() == GameState::InGame {
                  atlas.index = 0;
            } else if *state.get() == GameState::MainMenu {
                  atlas.index = 1;
            }
      }
}
