use sdl2::{render::Canvas, video::Window};

use crate::{sdl::sdl_load_textures, core::GameObject};

pub enum ScreenType {
    Win,
    Lose,
}

pub struct Screen {
    score: u32,
    _obj: GameObject,
}

impl Screen {
    pub fn new(canvas: &Canvas<Window>, scr_type: ScreenType) -> Self {
        let (scr_width, scr_height) = canvas.logical_size();
        let (sx, sy) = (scr_width as i32 / 2, scr_height as i32 / 2);
        let o = match scr_type {
            ScreenType::Win => GameObject::new(
                sdl_load_textures(canvas, vec![String::from("sprites/winner.png")]),
                sx,
                sy,
                true,
            ),
            ScreenType::Lose => GameObject::new(
                sdl_load_textures(canvas, vec![String::from("sprites/looser.png")]),
                sx,
                sy,
                true,
            ),
        };

        Self { score: 0, _obj: o }
    }
}
