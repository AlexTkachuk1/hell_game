use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
    pub player_layout: Option<Handle<TextureAtlasLayout>>,
    pub player_image: Option<Handle<Image>>,
    pub gun_image: Option<Handle<Image>>,
    pub coin_layout: Option<Handle<TextureAtlasLayout>>,
    pub coin_image: Option<Handle<Image>>,
    pub menu_layout: Option<Handle<TextureAtlasLayout>>,
    pub menu_image: Option<Handle<Image>>,
    pub cursor_layout: Option<Handle<TextureAtlasLayout>>,
    pub cursor_image: Option<Handle<Image>>,
    pub button_layout: Option<Handle<TextureAtlasLayout>>,
    pub button_image: Option<Handle<Image>>,
    pub castle_layout: Option<Handle<TextureAtlasLayout>>,
    pub castle_image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame))
            );
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    handle.player_image = Some(asset_server.load(PLAYER_SPRITE_SHEET_PATH));

    let player_layout = TextureAtlasLayout::from_grid(
        Vec2::new(PLAYER_TILE_W as f32, PLAYER_TILE_H as f32),
        PLAYER_SPRITE_SHEET_W,
        PLAYER_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.player_layout = Some(texture_atlas_layouts.add(player_layout));

    handle.gun_image = Some(asset_server.load(GUN_SPRITE_PATH));

    handle.coin_image = Some(asset_server.load(COIN_SPRITE_SHEET_PATH));

    let coin_layout = TextureAtlasLayout::from_grid(
        Vec2::new(COIN_TILE_W as f32, COIN_TILE_H as f32),
        COIN_SPRITE_SHEET_W,
        COIN_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.coin_layout = Some(texture_atlas_layouts.add(coin_layout));

    handle.menu_image = Some(asset_server.load(MENU_SPRITE_SHEET_PATH));

    let menu_layout = TextureAtlasLayout::from_grid(
        Vec2::new(MENU_TILE_W as f32, MENU_TILE_H as f32),
        MENU_SPRITE_SHEET_W,
        MENU_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.menu_layout = Some(texture_atlas_layouts.add(menu_layout));

    handle.cursor_image = Some(asset_server.load(CURSOR_SPRITE_SHEET_PATH));

    let cursor_layout = TextureAtlasLayout::from_grid(
        Vec2::new(CURSOR_TILE_W as f32, CURSOR_TILE_H as f32),
        CURSOR_SPRITE_SHEET_W,
        CURSOR_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.cursor_layout = Some(texture_atlas_layouts.add(cursor_layout));

    handle.button_image = Some(asset_server.load(BUTTON_SPRITE_SHEET_PATH));

    let button_layout = TextureAtlasLayout::from_grid(
        Vec2::new(BUTTON_TILE_W as f32, BUTTON_TILE_H as f32),
        BUTTON_SPRITE_SHEET_W,
        BUTTON_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.button_layout = Some(texture_atlas_layouts.add(button_layout));

    handle.castle_image = Some(asset_server.load(CASTLE_SPRITE_SHEET_PATH));

    let castle_layout = TextureAtlasLayout::from_grid(
        Vec2::new(CASTLE_TILE_W as f32, CASTLE_TILE_H as f32),
        CASTLE_SPRITE_SHEET_W,
        CASTLE_SPRITE_SHEET_H,
        None,
        None,
    );
    handle.castle_layout = Some(texture_atlas_layouts.add(castle_layout));

    next_state.set(GameState::MainMenu);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
            player_layout: None,
            player_image: None,
            gun_image: None,
            coin_layout: None,
            coin_image: None,
            menu_layout: None,
            menu_image: None,
            cursor_image: None,
            cursor_layout: None,
            button_image: None,
            button_layout: None,
            castle_image: None,
            castle_layout: None,
        }   
    }
}
