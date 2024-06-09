//Window
pub const WW: f32 = 1600.;
pub const WH: f32 = 900.;

//Assets
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;
pub const SPRITE_SHEET_W: usize = 8;
pub const SPRITE_SHEET_H: usize = 8;
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

pub const PLAYER_SPRITE_SHEET_PATH: &str = "player.png";
pub const PLAYER_SPRITE_SCALE_FACTOR: f32 = 0.6;
pub const PLAYER_TILE_W: usize = 268;
pub const PLAYER_TILE_H: usize = 268;
pub const PLAYER_SPRITE_SHEET_W: usize = 6;
pub const PLAYER_SPRITE_SHEET_H: usize = 4;

pub const COIN_SPRITE_SHEET_PATH: &str = "coin.png";
pub const COIN_SPRITE_SCALE_FACTOR: f32 = 0.15;
pub const COIN_TILE_W: usize = 188;
pub const COIN_TILE_H: usize = 220;
pub const COIN_SPRITE_SHEET_W: usize = 6;
pub const COIN_SPRITE_SHEET_H: usize = 1;

//World
pub const NUM_DECORRATIONS: usize = 3000;
pub const WORLD_W: f32 = 7000.0;
pub const WORLD_H: f32 = 7000.0;

//Player
pub const PLAYER_SPEED: f32 = 4.0;
pub const PLAYER_HEALTH: f32 = 100.0;

//Enemy
pub const MAX_NUM_ENEMIES: usize = 20;
pub const SPAWN_RATE_PER_SECOND: usize = 2;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.5;
pub const ENEMY_HEALTH: f32 = 10.0;
pub const ENEMY_DAMAGE: f32 = 1.0;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

//Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_SPEED: f32 = 20.0;
pub const BULLET_DAMAGE: f32 = 15.0;
pub const BULLET_TIME_SECS: f32 = 1.;
pub const NUM_BULLETS_PER_SHOT: usize = 10;

pub const GUN_SPRITE_SHEET_PATH: &str = "gun.png";
pub const GUN_SPRITE_SCALE_FACTOR: f32 = 1.0;
pub const GUN_TILE_W: usize = 201;
pub const GUN_TILE_H: usize = 142;
pub const GUN_SPRITE_SHEET_W: usize = 9;
pub const GUN_SPRITE_SHEET_H: usize = 3;

//Menu
pub const MENU_SPRITE_SHEET_PATH: &str = "bg.png";
pub const MENU_SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const MENU_TILE_W: usize = 800;
pub const MENU_TILE_H: usize = 450;
pub const MENU_SPRITE_SHEET_W: usize = 12;
pub const MENU_SPRITE_SHEET_H: usize = 21;

//Cursor
pub const CURSOR_SPRITE_SHEET_PATH: &str = "cursor.png";
pub const CURSOR_SPRITE_SCALE_FACTOR: f32 = 1.;
pub const CURSOR_TILE_W: usize = 112;
pub const CURSOR_TILE_H: usize = 112;
pub const CURSOR_SPRITE_SHEET_W: usize = 4;
pub const CURSOR_SPRITE_SHEET_H: usize = 1;

//Buttons
pub const BUTTON_SPRITE_SHEET_PATH: &str = "buttons.png";
pub const BUTTON_SPRITE_SCALE_FACTOR: f32 = 1.;
pub const BUTTON_TILE_W: usize = 164;
pub const BUTTON_TILE_H: usize = 41;
pub const BUTTON_SPRITE_SHEET_W: usize = 3;
pub const BUTTON_SPRITE_SHEET_H: usize = 1;

//Castle
pub const CASTLE_SPRITE_SHEET_PATH: &str = "castle.png";
pub const CASTLE_SPRITE_SCALE_FACTOR: f32 = 1.;
pub const CASTLE_TILE_W: usize = 300;
pub const CASTLE_TILE_H: usize = 300;
pub const CASTLE_SPRITE_SHEET_W: usize = 6;
pub const CASTLE_SPRITE_SHEET_H: usize = 5;
pub const CASTLE_HEALTH: f32 = 1000.0;
