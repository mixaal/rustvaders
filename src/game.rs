use std::time::{Duration, Instant};

use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    render::{Canvas, Texture},
    video::Window,
};

use crate::libsdl::{sdl_clear, sdl_init, sdl_maintain_fps, sdl_render_tex, CollisionBox, handle_collisions};

const FRAME_RATE : u32 = 60; //fps

const ENEMY_ROWS: u32 = 4;
const ENEMY_COLS: u32 = 10;

const ALIEN_VERT_SPEED: f32 = 12.5;    // falling spped, adjust according to the difficulty : 2.5 should be ok
const ALIEN_MISSILE_SPEED : i32 = 5;   // missile speed
const ALIEN_FIRING_RANGE : i32 = 40;   // alien fires when the player is in range - 10 is easier, 40 fires on a too broad range
const ALIEN_MISSILE_RATE : Duration = Duration::new(0, 1_000_000_000/3); // alien fire rate duration

const PLAYER_SPEED : i32 = 5;          // player horizontal speed
const PLAYER_MISSILE_SPEED : i32 = 5;  // player missile speed
const PLAYER_MISSILE_RATE : Duration = Duration::new(0, 1_000_000_000/3); // player fire rate duration


const FINAL_SCREEN_DUR : Duration = Duration::new(2, 0);

const MAX_ALIEN_MISSILES: i32 = 8;
const MAX_PLAYER_MISSILES: i32 = 8;
#[derive(PartialEq)]
enum ResourceType {
    Player, PlayerMissile, AlienMissile, Alien, WinPicture, LosePicture
}



pub struct Rustvaders {
    width: u32,
    height: u32,
    fps: u32,
    _animate: bool,
    _last_animate: Instant,
    _alien_vert_speed: f32,
    _alien_y_off: f32,
    _alien_resource_idx: i32,
    _player_horiz: i32,
    _player_vert: i32,
    _player_left: bool,
    _player_right: bool,
    _player_fire: bool,
    _player_last_fire: Instant,
    _alien_last_fire: Instant,
    _player_won: bool,
    _aliens_won: bool,
    _game_quit: bool,
    _game_end: Option<Instant>,
}

impl Rustvaders {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            fps: FRAME_RATE,
            _animate: false,
            _last_animate: Instant::now(),
            _alien_vert_speed: ALIEN_VERT_SPEED,
            _alien_y_off: 0.0,
            _alien_resource_idx: MAX_ALIEN_MISSILES + MAX_PLAYER_MISSILES,
            _player_horiz: 0,
            _player_vert: 0,
            _player_left: false,
            _player_right: false,
            _player_fire: false,
            _player_last_fire: Instant::now(),
            _alien_last_fire: Instant::now(),
            _player_won: false,
            _aliens_won: false,
            _game_quit: false,
            _game_end: None,
        }
    }

    pub fn mainloop(self: &mut Self) {
        let (mut event_pump, mut canvas) = sdl_init(self.width, self.height);

        let tc = canvas.texture_creator();
        let mut resources: Vec<(Vec<&Texture>, i32, i32, u32, u32, usize, bool, ResourceType)> = Vec::new();

        let octopus = tc.load_texture("sprites/chobotnice-modra.png").unwrap();
        let alien1 = tc.load_texture("sprites/hopsalek-01-zluty.png").unwrap();
        let alien2 = tc.load_texture("sprites/hopsalek-02-zluty.png").unwrap();
        let jelly_fish = tc.load_texture("sprites/meduza-cervena.png").unwrap();
        let canon = tc.load_texture("sprites/kanon.png").unwrap();
        let missile = tc.load_texture("sprites/strela-oranzova.png").unwrap();
        let alien_light = tc.load_texture("sprites/blesk-zluty.png").unwrap();
        let winner = tc.load_texture("sprites/winner.png").unwrap();
        let looser = tc.load_texture("sprites/looser.png").unwrap();
        

        let canon_x = self.width as i32 / 2;
        self._player_horiz = canon_x;
        let canon_y = (self.height - canon.query().height) as i32;
        self._player_vert = canon_y;
        resources.push((vec![&canon], canon_x, canon_y, canon.query().width, canon.query().height, 0, true, ResourceType::Player));
        for _ in 0..MAX_PLAYER_MISSILES {
            resources.push((vec![&missile], 0, canon_y, missile.query().width, missile.query().height, 0, false, ResourceType::PlayerMissile));
        }
        for _ in 0..MAX_ALIEN_MISSILES {
            resources.push((vec![&alien_light], 0, 0, missile.query().width, missile.query().height, 0, false, ResourceType::AlienMissile));
        }
        let p_dy: i32 = 10;
        let p_dx: i32 = (self.width / (ENEMY_COLS + 1)) as i32;

        let mut py: i32 = alien1.query().height as i32;
        for row in 1..=ENEMY_ROWS {
            let mut px = p_dx;
            for _ in 0..ENEMY_COLS {
                let (t, w, h) = match row {
                    1 => (vec![&octopus], octopus.query().width, octopus.query().height),
                    ENEMY_ROWS => (vec![&jelly_fish], jelly_fish.query().width, jelly_fish.query().height),
                    _ => (vec![&alien1, &alien2], alien1.query().width, alien1.query().height),
                };
                resources.push((t, px, py, w, h, 0, true, ResourceType::Alien));
                px += p_dx;
            }
            py += p_dy + alien1.query().height as i32;
        }

        resources.push((vec![&winner], self.width as i32 / 2, self.height as i32 / 2, winner.query().width, winner.query().height, 0, false, ResourceType::WinPicture));
        resources.push((vec![&looser], self.width as i32 / 2, self.height as i32 / 2, looser.query().width, looser.query().height, 0, false, ResourceType::LosePicture));

        let mut now = Instant::now();
        'running: loop {
            let start = Instant::now();
            // Handle events
            if self.keyhandler(&mut event_pump) {
                break 'running;
            }

            sdl_clear(&mut canvas, 0, 0, 0);
            if self.update(&mut resources, now.elapsed().as_secs_f32()) {
                // we either lost or won the game
                break 'running;
            }
            self.draw(&mut canvas, &mut resources);
            canvas.present();
            now = Instant::now();
            sdl_maintain_fps(start, self.fps);
        }
    }

    fn draw(
        self: &Self,
        canvas: &mut Canvas<Window>,
        resources: &mut Vec<(Vec<&Texture>, i32, i32, u32, u32, usize, bool, ResourceType)>,
        // (texture, x, y, animation_idx, visible)
    ) {

        if self._player_won {
            
           
        }

        if self._aliens_won {
           
        }

        for r in resources.iter_mut() {
            let res_type = &r.7;

            let end_of_game = self._player_won || self._aliens_won;
            if end_of_game {
                r.6 = false;
                if self._player_won && res_type==&ResourceType::WinPicture {
                    r.6 = true;
                }
                if self._aliens_won && res_type==&ResourceType::LosePicture {
                    r.6 = true;
                }
            }

            if r.6 { // visible
                if self._animate {
                    r.5 += 1; // animation index
                    if r.5 >= r.0.len() {
                        r.5 = 0;
                    }
                }
                let idx = r.5;
                // offset aliens by vertical fall off
                let y = if r.7 == ResourceType::Alien {
                    (r.2 as f32 + self._alien_y_off) as i32
                } else {
                    r.2
                };
                sdl_render_tex(canvas, &r.0[idx], r.1, y);
            }
        }
    }

    fn update(
        self: &mut Self,
        resources: &mut Vec<(Vec<&Texture>, i32, i32, u32, u32, usize, bool, ResourceType)>,
        dt: f32,
    )-> bool {
        // game end - wait a bit with final screen rendering
        if self._player_won || self._aliens_won {
    
            if self._game_end.is_some() && self._game_end.unwrap().elapsed() > FINAL_SCREEN_DUR {
                // quit the game
                self._game_quit = true;
            }

            return self._game_quit;
        }
        // aliens animation frames
        if self._last_animate.elapsed() > Duration::new(1, 0) {
            self._last_animate = Instant::now();
            self._animate = true;
        } else {
            self._animate = false;
        }

        // aliens adjust vertical offset: they are falling down 
        self._alien_y_off += self._alien_vert_speed * dt;

        // player wants to move right
        if self._player_right {
            self._player_horiz += PLAYER_SPEED;
            // circular arcade movement - appear on left side
            if self._player_horiz > self.width as i32 {
                self._player_horiz = 0;
            }
        }

        // player wants to move right
        if self._player_left {
            self._player_horiz -= PLAYER_SPEED;
            // circular arcade movement - appear on right side
            if self._player_horiz < 0 {
                self._player_horiz = self.width as i32;
            }
        }
        //alien fire
        let alien_fires = self._alien_last_fire.elapsed() > ALIEN_MISSILE_RATE;
        if alien_fires {
            self._alien_last_fire = Instant::now();
        }

        // decide if player fires, fire event from keyboard and check for missile rate !
        let player_fires = self._player_fire && self._player_last_fire.elapsed() > PLAYER_MISSILE_RATE;
        if player_fires {
            self._player_last_fire = Instant::now();
        }

        let mut missiles_col: Vec<CollisionBox> = Vec::new();
        let mut aliens_col : Vec<CollisionBox> = Vec::new();
        let mut alien_missiles_col: Vec<CollisionBox> = Vec::new();
        let mut players_col: Vec<CollisionBox> = Vec::new();
        let mut r_idx : usize = 0;
        let mut alien_max_y = 0;
        let mut alien_potential_firing_position = (0, 0);
        for r in resources.iter_mut() {
            let res_type = &r.7;
            let visible = r.6;
            let x = r.1;
            let y = if res_type == &ResourceType::Alien {
                (r.2 as f32 + self._alien_y_off) as i32
            } else {
                r.2
            };
            let w2 = r.3 as i32 / 2;
            let h2 = r.4 as i32 / 2;
            // update player's position
            if res_type == &ResourceType::Player && visible {
                r.1 = self._player_horiz;
                r.2 = self._player_vert;
                players_col.push(CollisionBox{index: r_idx, active:true, min_x: x-w2, min_y: y-h2, max_x: x+w2, max_y: y+h2});
            }
            // update all visible player missiles
            if res_type == &ResourceType::PlayerMissile && visible {
                r.2 -= PLAYER_MISSILE_SPEED;
                if r.2 < 0 {
                    r.6 = false; // dead
                }
                
                missiles_col.push(CollisionBox{index: r_idx, active:true, min_x: x-w2, min_y: y-h2, max_x: x+w2, max_y: y+h2});
            }

            // update all visible alien missiles
            if res_type == &ResourceType::AlienMissile && visible {
                r.2 += ALIEN_MISSILE_SPEED;
                if r.2 > self.height as i32 {
                    r.6 = false; // dead
                }
                
                alien_missiles_col.push(CollisionBox{index: r_idx, active:true, min_x: x-w2, min_y: y-h2, max_x: x+w2, max_y: y+h2});
            }
            
            // collisions for remaining aliens, decide on firing
            if res_type == &ResourceType::Alien && visible {
                aliens_col.push(CollisionBox{index: r_idx, active:true, min_x: x-w2, min_y: y-h2, max_x: x+w2, max_y: y+h2});
                if y+h2 > alien_max_y {
                    alien_max_y = y + h2;
                }
                if (x - self._player_horiz).abs() < ALIEN_FIRING_RANGE { 
                    alien_potential_firing_position = (x, y);
                }
            }

            r_idx += 1;
        }

        // no players left, aliens won
        let playrs_left = players_col.len();
        if playrs_left == 0 {
            self._aliens_won = true;
        }

        let aliens_left = aliens_col.len();
        if aliens_left == 0 {
            self._player_won = true;
        }
        // alien winning condition - reaching player
        if alien_max_y + 50 > self._player_vert {
            self._aliens_won = true;
        }
        // handle collisions - check what collided
        let collisions = handle_collisions(&mut missiles_col, &mut aliens_col);
        let alien_collisions = handle_collisions(&mut alien_missiles_col, &mut players_col);

        // handle collisions - destroy resources
        for c in collisions.iter() {
            resources[c.0].6 = false; //missile dead
            resources[c.1].6 = false; //alien dead
        }

        for c in alien_collisions {
            resources[c.0].6 = false; // alien missile dead
            resources[c.1].6 = false; // player dead
        }

        if player_fires  {
            // find first free player missile slot and 
            for r in resources.iter_mut() { 
                let res_type = &r.7;
                let visible = r.6;
                if res_type == &ResourceType::PlayerMissile && !visible {
                    // got new missile, initialize it
                    r.1 = self._player_horiz;
                    r.2 = self._player_vert;
                    r.6 = true;
                    break;
                }
            }
        }

        if alien_fires && alien_potential_firing_position.0 > 0  {
            for r in resources.iter_mut() { 
                let res_type = &r.7;
                let visible = r.6;
                if res_type == &ResourceType::AlienMissile && !visible {
                    // got new alien missile, initialize it
                    r.1 = alien_potential_firing_position.0;
                    r.2 = alien_potential_firing_position.1;
                    r.6 = true;
                    break;
                }
            }
        }

        self._game_end = match self._game_end {
            None if self._player_won || self._aliens_won => Some(Instant::now()),
            _ => None,
        };

        // println!("game_end: {:?}", self._game_end);

        self._game_quit
    }

    

    fn keyhandler(self: &mut Self, event_pump: &mut sdl2::EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self._player_left = true;
                    self._player_right = false;
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self._player_left = false;
                    self._player_right = true;
                    return false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self._player_left = false;
                    return false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self._player_right = false;
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if !self._player_fire {
                        self._player_fire = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self._player_fire = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    println!("Pausing Music");
                    // sound_manager.stop_sound(&MUSIC_FILENAME.to_string());
                }
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    println!("Resuming Music");
                    // sound_manager.resume_sound(&MUSIC_FILENAME.to_string());
                }
                _ => {}
            }
        }
        return false;
    }
}
