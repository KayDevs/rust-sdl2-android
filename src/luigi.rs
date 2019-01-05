use sprite::{Sprite, Render};
use entity::{Entity, GameState};
use hitbox::{HitBox, Collide};

use data;

pub struct Murph {
    health: u8,
    sprite: Sprite,
}

impl Render for Murph {
    fn get_sprites(&self) -> Vec<&Sprite> {
        return vec![&self.sprite];
    }
}

impl Collide for Murph {
    fn get_hitboxes(&self) -> Vec<HitBox> {
        return vec![HitBox::new(self.sprite.x, self.sprite.y, self.sprite.w, self.sprite.h)];
    }
    fn handle_collision(&mut self, ce: CollisionEvent) {
        match ce.impact {
            0 ... => {
                self.health -= ce.impact;
                self.sprite.x -= ce.direction.0;
                self.sprite.y -= ce.direction.1;
            },
            -1 => { println!("hello!"); },
            _ => {},
        }
    }
}

impl Entity for Murph {
    fn new() -> Self {
        let mut m = Murph{health: 255, sprite: Sprite::new(data::Murph)};
        m.sprite.w = 16; //400
        m.sprite.h = 16; //315
        return m;
    }
    fn tick(&mut self, gs: &GameState) {
        if gs.primary {
            self.sprite.flip_v = true;
        } else {
            self.sprite.flip_v = false;
        }
        if gs.up {
            self.sprite.y -= 1;
        }
        if gs.down {
            self.sprite.y += 1;
        }
        if gs.left {
            self.sprite.x -= 1;
            self.sprite.anim_index += 0.25;
            self.sprite.flip_h = true;
        } else if gs.right {
            self.sprite.x += 1;
            self.sprite.anim_index += 0.25;
            self.sprite.flip_h = false;
        } else {
            self.sprite.anim_index = 0.0;
        }
    }
}
