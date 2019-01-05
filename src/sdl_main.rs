extern crate libc;
extern crate sdl2;

mod data;

mod entity;
use entity::{Entity, GameState};

mod sprite;
use sprite::{Render, SpriteCache};

mod hitbox;
use hitbox::{Collide, HitBox};

mod murph;
use murph::Murph;

mod brick;
use brick::Brick;

use std::cell::RefCell;
use std::rc::Rc;

//use behaviour trees for AI and such

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    main();
    return 0;
}

pub fn main() {
    //initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("demo", 256, 224)
        .fullscreen_desktop()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");
    canvas.set_logical_size(256, 224).unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 255));
    let textures = canvas.texture_creator();
    let mut sprite_cache = SpriteCache::new();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //initialize objects
    let mut gs: GameState = GameState {
        up: false,
        down: false,
        left: false,
        right: false,
        primary: false,
        secondary: false,
    };
    let mut entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
    let mut renders: Vec<Rc<RefCell<Render>>> = Vec::new();
    let mut collides: Vec<Rc<RefCell<Collide>>> = Vec::new();

    let map: Vec<Vec<u8>> =
        vec![vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1],
             vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
             vec![1, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
             vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
             vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
             vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
             vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],];
    
    //game needs a player
    let murph: Rc<RefCell<Murph>> = Rc::new(RefCell::new(Murph::new()));
    //get the other objects
    for (y, i) in map.iter().enumerate() {
        for (x, j) in i.iter().enumerate() {
            match j {
                2 => {
                    entities.push(murph.clone());
                    renders.push(murph.clone());
                    collides.push(murph.clone());
                    let newpos = renders.len() - 1;
                    renders[newpos].borrow_mut().get_sprite().x = (x * 16) as f32;
                    renders[newpos].borrow_mut().get_sprite().y = (y * 16) as f32;
                },
                1 => {
                    let rcl = Rc::new(RefCell::new(Brick::new()));
                    renders.push(rcl.clone());
                    collides.push(rcl.clone());
                    let newpos = renders.len() - 1;
                    renders[newpos].borrow_mut().get_sprite().x = (x * 16) as f32;
                    renders[newpos].borrow_mut().get_sprite().y = (y * 16) as f32;
                },
                _ => {},
            }
        }
    }
        
    'game: loop {
        //input
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'game,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => gs.up = true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Down),
                    ..
                } => gs.down = true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => gs.left = true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Right),
                    ..
                } => gs.right = true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Z),
                    ..
                } => gs.primary = true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::X),
                    ..
                } => gs.secondary = true,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => gs.up = false,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Down),
                    ..
                } => gs.down = false,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => gs.left = false,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Right),
                    ..
                } => gs.right = false,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Z),
                    ..
                } => gs.primary = false,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::X),
                    ..
                } => gs.secondary = false,
                _ => {}
            }
        }
        
        //update
        for e in &mut entities {
            e.borrow_mut().tick(&gs);
        }
        
        //collisions
        for (i, c) in collides.iter().enumerate() {
            for (j, d) in collides.iter().enumerate() {
                if i == j {
                    //don't collide with self
                    continue;
                }
                let same: HitBox = c.borrow().get_hitbox();
                let other: HitBox = d.borrow().get_hitbox();
                if hitbox::box_collision(same, other) {
                    let collision = d.borrow().collision(same);
                    c.borrow_mut().handle_collision(collision, other);
                    //break;
                }
            }
        }

        //render
        canvas.clear();
        {
            let mut m = murph.borrow_mut();
            canvas.set_viewport(sdl2::rect::Rect::new(
                -m.get_sprite().x as i32 + 128,
                -m.get_sprite().y as i32 + 112,
                256 * 4,
                224 * 4));
        }
        for r in &mut renders {
            r.borrow_mut().get_sprite().render(&mut sprite_cache, &textures, &mut canvas);
        }
        canvas.present();
    }
}
