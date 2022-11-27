use std::time::Duration;

pub mod aliens;
pub mod core;
pub mod game;
pub mod player;
pub mod screens;
pub mod sdl;
pub mod timer;

const FRAME_RATE: u32 = 60; //fps

const ENEMY_ROWS: u32 = 4;
const ENEMY_COLS: u32 = 10;

const ALIEN_VERT_SPEED: f32 = 12.5; // falling spped, adjust according to the difficulty : 2.5 should be ok
const ALIEN_MISSILE_SPEED: i32 = 500; // missile speed
const ALIEN_FIRING_RANGE: i32 = 40; // alien fires when the player is in range - 10 is easier, 40 fires on a too broad range
const ALIEN_MISSILE_RATE: Duration = Duration::from_millis(330); // alien fire rate duration

const PLAYER_SPEED: f32 = 600.0; // player horizontal speed
const PLAYER_MISSILE_SPEED: i32 = 500; // player missile speed
const PLAYER_MISSILE_RATE: Duration = Duration::from_millis(330); // player fire rate duration

const FINAL_SCREEN_DUR: Duration = Duration::new(2, 0);

const MAX_ALIEN_MISSILES: usize = 8;
const MAX_PLAYER_MISSILES: usize = 8;
