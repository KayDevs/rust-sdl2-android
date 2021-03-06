extern crate sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use std::collections::HashMap;

pub trait Render {
    fn get_sprite(&mut self) -> &mut Sprite;
}

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub anim_sprite: u32,
    pub anim_index: f64, //for slowed down animations round up
    pub flip_h: bool,
    pub flip_v: bool,
    pub angle: f64,
    tex: &'static [u8],
}

pub struct SpriteCache<'a> {
    textures: HashMap<&'static [u8], Texture<'a>>,
}

impl<'a> SpriteCache<'a> {
    pub fn new() -> SpriteCache<'a> {
        SpriteCache {
            textures: HashMap::new(),
        }
    }
    
    //load assets as-needed during gameplay
    pub fn get_or_load_texture<T>(&mut self, index: &'static [u8], tc: &'a TextureCreator<T>) -> &Texture<'a> {
        self.textures.entry(index).or_insert({
            let mut surface = Surface::load_bmp_rw(&mut RWops::from_bytes(index).unwrap()).unwrap();
            surface.set_color_key(true, sdl2::pixels::Color {r: 255, g: 0, b: 255, a: 255,}).unwrap();
            tc.create_texture_from_surface(surface).unwrap()
        })
    }
}

impl Sprite {
    pub fn new(index: &'static [u8]) -> Sprite {
        Sprite {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            anim_sprite: 0,
            anim_index: 0.0,
            flip_h: false,
            flip_v: false,
            angle: 0.0,
            tex: index,
        }
    }
}

impl<'a> Sprite {
    pub fn render<T, U: RenderTarget>(&self, sc: &mut SpriteCache<'a>, tc: &'a TextureCreator<T>, c: &mut Canvas<U>) {
        let tex: &Texture<'a> = sc.get_or_load_texture(self.tex, tc);
        c.copy_ex(
            &tex,
            Rect::new(
                (self.anim_index as u32 * self.w as u32 % tex.query().width) as i32,
                (self.anim_sprite * self.h as u32 % tex.query().height) as i32,
                self.w as u32,
                self.h as u32,
            ),
            Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32),
            self.angle,
            Point::new(self.w as i32 / 2, self.h as i32 / 2),
            self.flip_h,
            self.flip_v,
        ).unwrap();
    }
}
