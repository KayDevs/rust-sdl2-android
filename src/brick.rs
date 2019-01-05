use sprite::{Render, Sprite};
use entity::{Entity, GameState};
use hitbox;
use hitbox::{HitBox, Collide, CollisionEvent};

use data;

pub struct Brick {
    sprite: Sprite,
}

impl Render for Brick {
    fn get_sprite(&mut self) -> &mut Sprite {
        return &mut self.sprite;
    }
}

impl Entity for Brick {
    fn new() -> Self {
        let mut b = Brick{sprite: Sprite::new(data::BRICK)};
        b.sprite.w = 16.0;
        b.sprite.h = 16.0;
        return b;
    }
    fn tick(&mut self, gs: &GameState) {
        //nothing to be done :o
    }
}

impl Collide for Brick {
    fn get_hitbox(&self) -> HitBox {
        return HitBox{x: self.sprite.x, y: self.sprite.y, w: self.sprite.w, h: self.sprite.h};
    }
    fn collision(&self, other: HitBox) -> CollisionEvent {
        return CollisionEvent{impact: 0, direction: hitbox::box_mtv(self.get_hitbox(), other)};
    }
    fn handle_collision(&mut self, _: CollisionEvent, _: HitBox) {
        //do nothing
    }
}
