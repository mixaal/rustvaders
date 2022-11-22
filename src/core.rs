use std::time::Duration;

use sdl::sdl_render_tex;
use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use crate::{sdl, timer::GameTimer};

pub trait Drawable {
    fn draw(&mut self, canvas: &mut Canvas<Window>);
}

pub trait Audible {
    fn play(&self);
}

pub struct GameObject {
    textures: Vec<Texture>,
    texno: usize,
    anim_ms: Duration,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub alive: bool,
    _anim_idx: usize,
    _anim_timer: GameTimer,
    _collision_box: CollisionBox,
}

impl GameObject {
    pub fn new(textures: Vec<Texture>, x: i32, y: i32, alive: bool) -> Self {
        let texno = textures.len();
        let has_tex = !textures.is_empty();
        let w = if has_tex {
            textures[0].query().width
        } else {
            0
        };
        let h = if has_tex {
            textures[0].query().height
        } else {
            0
        };
        let anim_ms = Duration::from_millis(50);
        Self {
            textures,
            texno,
            anim_ms,
            x,
            y,
            w,
            h,
            alive,
            _anim_idx: 0,
            _anim_timer: GameTimer::new(anim_ms),
            _collision_box: CollisionBox::new(x, y, w, h),
        }
    }

    pub fn anim_speed(&mut self, anim_ms: Duration) {
        self.anim_ms = anim_ms;
        self._anim_timer = GameTimer::new(anim_ms);
    }

    pub fn next_costume(&mut self) {
        self._anim_idx += 1;
        self._anim_idx %= self.texno;
    }

    pub fn animate(&mut self) {
        self._anim_timer.update();
        if self._anim_timer.ready {
            self._anim_timer.reset();
            self.next_costume();
        }
    }

    pub fn dist2(&self, other: &GameObject) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy) as f32
    }

    pub fn update_collision_box(&mut self) {
        self._collision_box.update(self.x, self.y);
    }

    pub fn resolve_collision(&mut self, other: &mut GameObject) -> bool {
        if !self.alive {
            return  false;
        }

        if !other.alive {
            return  false;
        }
        self._collision_box.resolve_collision(&mut other._collision_box)
    }
}

impl Drawable for GameObject {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if self.alive {
            let tex = &self.textures[self._anim_idx];
            sdl_render_tex(canvas, tex, self.x, self.y);
        }
    }
}

pub struct CollisionBox {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    _w2: i32,
    _h2: i32,
}

impl CollisionBox {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        let w2 = w as i32 / 2;
        let h2 = h as i32 /  2;
        Self {
            min_x: x - w2, min_y: y - h2, max_x: x + w2, max_y: y + h2, _w2: w2, _h2: h2,
        }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.min_x = x - self._w2;
        self.min_y = y - self._h2;
        self.max_x = x + self._w2;
        self.max_y = y + self._h2;
    }

    pub fn collides(&self, other: &CollisionBox) -> bool {
        self.min_x < other.max_x && self.max_x > other.min_x && self.min_y < other.max_y && self.max_y > other.min_y
    }

    pub fn resolve_collision(&mut self, other: &mut CollisionBox) -> bool {
        self.collides(other)
    }
}