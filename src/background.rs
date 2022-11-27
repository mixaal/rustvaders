use sdl2::{render::Canvas, video::Window};

use crate::{core::GameObject, sdl::sdl_load_textures};

pub struct Background {
    _vert_pos: f32,
    _scr_h: u32,
    scroll_speed: f32,
    pub render: GameObject,
}

impl Background {
    pub fn new(canvas: &Canvas<Window>, scr_width: u32, scr_height: u32) -> Self {
        let mut o = GameObject::new(
            sdl_load_textures(canvas, vec![String::from("sprites/earth-bg.jpg")]),
            scr_width as i32 / 2,
            0,
            true,
        );
        let h = 100;
        // adjust player vertical position
        o.y = h;
        Self {
            scroll_speed: 100.0,
            render: o,
            _vert_pos: h as f32,
            _scr_h: scr_height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let min_vert = self._scr_h as f32 - self.render.h as f32 / 2.0;
        if self._vert_pos > min_vert {
            self._vert_pos -= self.scroll_speed * dt;
            self.render.y = self._vert_pos as i32;
        }
    }
}
