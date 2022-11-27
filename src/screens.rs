use sdl2::{render::Canvas, video::Window};

use crate::{core::GameObject, sdl::sdl_load_textures, timer::GameTimer, FINAL_SCREEN_DUR};

pub enum ScreenType {
    Win,
    Lose,
}

pub struct Screen {
    score: u32,
    pub render: GameObject,
    pub timer: GameTimer,
}

impl Screen {
    pub fn new(
        canvas: &Canvas<Window>,
        scr_width: u32,
        scr_height: u32,
        scr_type: ScreenType,
    ) -> Self {
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

        Self {
            score: 0,
            render: o,
            timer: GameTimer::new(FINAL_SCREEN_DUR),
        }
    }

    pub fn update(&mut self) {
        self.timer.update();
    }
}
