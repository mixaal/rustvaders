use std::time::{Duration, Instant};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
    EventPump,
};

pub fn sdl_init(width: u32, height: u32) -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rustvaders", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();
    (event_pump, canvas)
}

pub fn sdl_load_textures(canvas: &Canvas<Window>, images: Vec<String>) -> Vec<Texture> {
    let mut textures: Vec<Texture> = Vec::new();
    let tc = canvas.texture_creator();
    for img in images.iter() {
        let tex = tc.load_texture(img).unwrap();
        textures.push(tex);
    }
    textures
}

pub fn sdl_render_tex(canvas: &mut Canvas<Window>, texture: &Texture, x: i32, y: i32) {
    let h = texture.query().height;
    let w = texture.query().width;

    let sprite = Rect::new(0, 0, w, h);
    canvas
        .copy(
            texture,
            sprite,
            Rect::from_center(Point::new(x, y), sprite.width(), sprite.height()),
        )
        .unwrap();
}

pub fn sdl_clear(canvas: &mut Canvas<Window>, r: u8, g: u8, b: u8) {
    canvas.set_draw_color(Color::RGB(r, g, b));
    canvas.clear();
}

pub fn sdl_maintain_fps(start: Instant, fps: u32) {
    let frame_duration = Duration::new(0, 1_000_000_000u32 / fps);
    let elapsed = start.elapsed();
    match frame_duration.checked_sub(elapsed) {
        Some(dt) => ::std::thread::sleep(dt),
        None => {}
    }
}

pub struct CollisionBox {
    pub index: usize,
    pub active: bool,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}
