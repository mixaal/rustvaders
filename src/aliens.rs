use std::time::Duration;

use sdl2::{render::Canvas, video::Window};

use crate::{sdl::sdl_load_textures, core::GameObject, ALIEN_VERT_SPEED, ALIEN_MISSILE_SPEED, timer::GameTimer, ALIEN_FIRING_RANGE, ALIEN_MISSILE_RATE, player::Player};

pub enum AlienType {
    Octopus,
    Jumper,
    JellyFish,
}

pub struct Alien {
    pub render: GameObject,
    _vert: f32, 
}

impl Alien {
    pub fn new(canvas: &Canvas<Window>, alien_type: AlienType, x: f32, y: f32) -> Self {
        let images = match alien_type {
            AlienType::Octopus => vec![String::from("sprites/chobotnice-cervena.png")],
            AlienType::JellyFish => vec![String::from("sprites/meduza-modra.png")],
            AlienType::Jumper => vec![
                String::from("sprites/hopsalek-01-zluty.png"),
                String::from("sprites/hopsalek-02-zluty.png"),
            ],
        };
        let mut render = GameObject::new(sdl_load_textures(canvas, images), x as i32, y as i32, true);
        render.anim_speed(Duration::from_millis(500));
        Self {
            render,
            _vert: y as f32,
            
        }
    }

    pub fn update(&mut self, dt: f32) {
        self._vert += ALIEN_VERT_SPEED * dt;
        self.render.y = self._vert as i32;
        self.render.update_collision_box();
    }


    pub fn die(&mut self) {
        self.render.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.render.alive
    }

    pub fn get_x(&self) -> i32 {
        self.render.x
    }
    pub fn get_y(&self) -> i32 {
        self.render.y
    }
}

pub struct AlienMissile {
    speed: f32,
    pub render: GameObject,
    pub _vert: f32, 
}

impl AlienMissile {
    pub fn new(canvas: &Canvas<Window>, speed: f32, position: (i32, i32)) -> Self {
        Self {
            speed,
            render: GameObject::new(
                sdl_load_textures(canvas, vec![String::from("sprites/blesk-zluty.png")]),
                position.0,
                position.1,
                true,
            ),
            _vert: position.1 as f32,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self._vert += self.speed * dt;
        self.render.y = self._vert as i32;
        self.render.update_collision_box();
    }

    pub fn resolve_collision(&mut self, player: &mut Player) {
        if self.render.resolve_collision(&mut player.render) {
            self.render.alive = false;
            player.die();
        }
    }
}
