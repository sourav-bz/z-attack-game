//window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

//sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 8;
pub const SPRITE_SHEET_H: u32 = 8;

//world
pub const NUM_WORLD_DECORATIONS: u32 = 1000;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;

//player
pub const PLAYER_SPEED: f32 = 4.0;

//enemy
pub const MAX_NUM_ENEMIES: u32 = 500;
pub const SPAWN_RATE_PER_SECOND : u32 = 100;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 2.0;
pub const ENEMY_HEALTH: f32 = 100.0;

//bullet
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_SPEED: f32 = 15.0;
pub const BULLET_DAMAGE: f32 = 100.0;
pub const BULLET_LIFE_TIME_IN_SECS: f32 = 0.8;
pub const NUM_OF_BULLET_PER_SHOT: u32 = 3;
