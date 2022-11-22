use std::time::{Duration, Instant};

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};

use crate::{
    aliens::{Alien, AlienMissile, AlienType},
    core::Drawable,
    player::{Player, PlayerMissile},
    sdl::{handle_collisions, sdl_clear, sdl_init, sdl_maintain_fps, sdl_render_tex, CollisionBox},
    timer::GameTimer,
    ALIEN_FIRING_RANGE, ALIEN_MISSILE_RATE, ALIEN_MISSILE_SPEED, ALIEN_VERT_SPEED, ENEMY_COLS,
    ENEMY_ROWS, FINAL_SCREEN_DUR, FRAME_RATE, MAX_ALIEN_MISSILES, MAX_PLAYER_MISSILES,
    PLAYER_MISSILE_RATE, PLAYER_MISSILE_SPEED, PLAYER_SPEED,
};

pub struct Rustvaders {
    width: u32,
    height: u32,
    fps: u32,
    // game objects
    _players: Vec<Player>,
    _aliens: Vec<Alien>,
    _player_missiles: Vec<PlayerMissile>,
    _alien_missiles: Vec<AlienMissile>,
    //events
    _player_left: bool,
    _player_right: bool,
    _player_fires: bool,
    _player_fire_timer: GameTimer,
    _alien_fire_timer: GameTimer,
}

impl Rustvaders {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            fps: FRAME_RATE,
            _players: Vec::new(),
            _aliens: Vec::new(),
            _player_missiles: Vec::new(),
            _alien_missiles: Vec::new(),
            _player_left: false,
            _player_right: false,
            _player_fires: false,
            _player_fire_timer: GameTimer::new(PLAYER_MISSILE_RATE),
            _alien_fire_timer: GameTimer::new(ALIEN_MISSILE_RATE),
        }
    }

    pub fn mainloop(&mut self) {
        let (mut event_pump, mut canvas) = sdl_init(self.width, self.height);

        // setup player, missiles, alien missiles, aliens and final screens

        self._players
            .push(Player::new(&canvas, self.width, self.height));

        let p_dy: i32 = 10;
        let p_dx: i32 = (self.width / (ENEMY_COLS + 1)) as i32;

        let mut py: i32 = 64;
        for row in 1..=ENEMY_ROWS {
            let mut px = p_dx;
            for _ in 0..ENEMY_COLS {
                let alien_type = match row {
                    1 => AlienType::Octopus,
                    ENEMY_ROWS => AlienType::JellyFish,
                    _ => AlienType::Jumper,
                };
                self._aliens
                    .push(Alien::new(&canvas, alien_type, px as f32, py as f32));
                px += p_dx;
            }
            py += 64;
        }

        // main loop
        let mut now = Instant::now();
        'running: loop {
            let start = Instant::now();
            // handle keyboard events
            if self.keyhandler(&mut event_pump) {
                break 'running;
            }
            // clear before drawing
            sdl_clear(&mut canvas, 0, 0, 0);

            // update game, if true returned, game ends
            self.update(&canvas, now.elapsed().as_secs_f32());

            // finally draw the game and maintain fps
            self.draw(&mut canvas);
            canvas.present();
            now = Instant::now();
            sdl_maintain_fps(start, self.fps);
        }
    }

    fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        // (texture, x, y, animation_idx, visible)
    ) {
        // iterate through all resources and draw them if visible
        for p in self._players.iter_mut() {
            p.render.draw(canvas);
        }

        for a in self._aliens.iter_mut() {
            a.render.draw(canvas);
        }

        for m in self._player_missiles.iter_mut() {
            m.render.draw(canvas);
        }

        for m in self._alien_missiles.iter_mut() {
            m.render.draw(canvas);
        }
    }

    fn update(&mut self, canvas: &Canvas<Window>, dt: f32) {
        let player = match self._players.len() {
            0 => None,
            _ => Some(&mut self._players[0]),
        };
        let mut px = 0;
        let has_player = player.is_some();
        if  has_player {
            let p = player.unwrap();
            px = p.render.x;
            if self._player_left {
                p.move_left();
            }

            if self._player_right {
                p.move_right();
            }
        }

        self._player_fire_timer.update();

        if has_player && self._player_fires {
            if self._player_fire_timer.ready {
                self._player_fire_timer.reset();

                if self._player_missiles.len() < MAX_PLAYER_MISSILES {
                    self._player_missiles.push(PlayerMissile::new(
                        canvas,
                        PLAYER_MISSILE_SPEED as f32,
                        &self._players[0],
                    ));
                }
            }
        }

        let mut alien_potential_firing_position = (0, 0);
        for alien in self._aliens.iter_mut() {
            alien.update(dt);
            alien.render.animate();
            if has_player && (alien.render.x - px).abs() < ALIEN_FIRING_RANGE {
                alien_potential_firing_position = (alien.render.x, alien.render.y);
            }
        }

        self._alien_fire_timer.update();
        if self._alien_fire_timer.ready
            && alien_potential_firing_position.0 > 0
            && self._alien_missiles.len() < MAX_ALIEN_MISSILES
        {
            self._alien_fire_timer.reset();
            self._alien_missiles.push(AlienMissile::new(
                canvas,
                ALIEN_MISSILE_SPEED as f32,
                alien_potential_firing_position,
            ));
        }

        for missile in self._player_missiles.iter_mut() {
            missile.update(dt);
            for alien in self._aliens.iter_mut() {
                missile.resolve_collision(alien);
            }
        }

        for missile in self._alien_missiles.iter_mut() {
            missile.update(dt);
            for player in self._players.iter_mut() {
                missile.resolve_collision(player);
            }
        }

        self.remove_objects();
    }

    fn remove_objects(&mut self) {
        self._alien_missiles
            .retain(|m| m._vert < self.height as f32);
        self._player_missiles.retain(|m| m._vert > 0.0);
        self._players.retain(|p| p.alive());
        self._aliens.retain(|p| p.alive());
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
                    if !self._player_fires {
                        self._player_fires = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self._player_fires = false;
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
