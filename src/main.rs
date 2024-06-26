use bevy::prelude::*;
use bevy::window::close_on_esc;

use hell_game::castle::CastlePlugin;
use hell_game::cursor::CursorPlugin;
use hell_game::gold::GoldPlugin;
use hell_game::gui::GuiPlugin;
use hell_game::collision::CollisionPlugin;
use hell_game::enemy::EnemyPlagin;
use hell_game::animation::AnimationPlugin;
use hell_game::camera::FollowCameraPlugin;
use hell_game::gun::GunPlugin;
use hell_game::player::PlayerPlugin;
use hell_game::state::GameState;
use hell_game::world::WorldPlugin;
use hell_game::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: false,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CastlePlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlagin)
        .add_plugins(CursorPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(GoldPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Update, close_on_esc)
        .run();
}
