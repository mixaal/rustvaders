use sdl2::{render::Canvas, video::Window};

use crate::{aliens::Alien, core::GameObject, sdl::sdl_load_textures, PLAYER_SPEED};

pub struct Player {
    pub score: u32,
    pub render: GameObject,
    _scr_w: u32,
}

impl Player {
    pub fn new(canvas: &Canvas<Window>, scr_width: u32, scr_height: u32) -> Self {
        let mut o = GameObject::new(
            sdl_load_textures(canvas, vec![String::from("sprites/kanon.png")]),
            scr_width as i32 / 2,
            0,
            true,
        );
        let h = o.h;
        // adjust player vertical position
        o.y = (scr_height - h) as i32;
        Self {
            score: 0,
            render: o,
            _scr_w: scr_width,
        }
    }

    pub fn alive(&self) -> bool {
        self.render.alive
    }

    pub fn move_right(&mut self) {
        self.render.x += PLAYER_SPEED;
        if self.render.x > self._scr_w as i32 {
            self.render.x = 0;
        }
        self.render.update_collision_box();
    }

    pub fn move_left(&mut self) {
        self.render.x -= PLAYER_SPEED;
        if self.render.x < 0 {
            self.render.x = self._scr_w as i32;
        }
        self.render.update_collision_box();
    }

    pub fn die(&mut self) {
        self.render.alive = false;
    }

    pub fn get_x(&self) -> i32 {
        self.render.x
    }
    pub fn get_y(&self) -> i32 {
        self.render.y
    }
}

pub struct PlayerMissile {
    speed: f32,
    pub render: GameObject,
    pub _vert: f32,
}

impl PlayerMissile {
    pub fn new(canvas: &Canvas<Window>, speed: f32, player: &Player) -> Self {
        Self {
            speed,
            render: GameObject::new(
                sdl_load_textures(canvas, vec![String::from("sprites/strela-oranzova.png")]),
                player.get_x(),
                player.get_y(),
                true,
            ),
            _vert: player.get_y() as f32,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self._vert -= self.speed * dt;
        self.render.y = self._vert as i32;
        self.render.update_collision_box();
    }

    pub fn resolve_collision(&mut self, alien: &mut Alien) {
        if self.render.resolve_collision(&mut alien.render) {
            self.render.alive = false;
            alien.die();
        }
    }
}
