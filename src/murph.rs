use entity::{Entity, GameState};
use hitbox;
use hitbox::{Collide, CollisionEvent, HitBox};
use sprite::{Render, Sprite};

use data;

pub struct Murph {
    health: i32,
    sprite: Sprite,

    //physics
    velx: f32,
    vely: f32,
    termx: f32,
    termy: f32,

    //collision variables
    blocked_left: bool,
    blocked_right: bool,
    blocked_above: bool,
    blocked_ground: bool,
    difference_h: f32,
    difference_v: f32,
}

impl Render for Murph {
    fn get_sprite(&mut self) -> &mut Sprite {
        return &mut self.sprite;
    }
}

impl Collide for Murph {
    fn get_hitbox(&self) -> HitBox {
        return HitBox {
            x: self.sprite.x + self.velx, //"ahead of time" collision testing
            y: self.sprite.y + self.vely,
            w: self.sprite.w,
            h: self.sprite.h,
        };
    }
    fn handle_collision(&mut self, ce: CollisionEvent, other: HitBox) {
        let hitbox = self.get_hitbox();
        if other.x < hitbox.x {
            self.blocked_left = true;
            self.difference_h = (other.x + other.w) - self.sprite.x;
        }
        if other.x > hitbox.x {
            self.blocked_right = true;
            self.difference_h = other.x - (self.sprite.x + self.sprite.w);
        }
        if other.y < hitbox.y {
            self.blocked_above = true;
            self.difference_v = (other.y + other.h) - self.sprite.y;
        }
        if other.y > hitbox.y {
            self.blocked_ground = true;
            self.difference_v = other.y - (self.sprite.y + self.sprite.h);
        }
    }
    /*fn handle_collision_legacy(&mut self, ce: CollisionEvent, other: HitBox) {
        match ce.impact {
            i if i > 0 => {
                //something hit him
                self.health -= i;
            }
            0 => {
                //wall

                /* 
                player doesn't have a single hitbox, but when checking walls is actually
                split into a 'cross' of two thin horizontal and vertical hitboxes.
                the size of these hitboxes is relative to his terminal velocities in either direction.
                this means the horizontal hitbox can check all horizontal collisions,
                and the vertical hitbox can check vertical collisions.
                This is only useful for walls, which the player must respond to.
                enemies, items, etc. just need the more basic bounding box tests.
                */

                //prioritize vertical collisions
                let mut vertical: HitBox = self.get_hitbox();
                vertical.x += (self.termx) / 2.0;
                vertical.w -= self.termx;
                //println!("vertical hitbox: {},{},{},{}", vertical.x - self.sprite.x, vertical.y - self.sprite.y, vertical.w, vertical.h);
                if hitbox::box_collision(other, vertical) {
                    let (dx, dy) = hitbox::box_overlap(other, vertical);
                    //hack: ignore collision responses that are greater than that which he can actually physically move...
                    //it's a bug and I don't know why he gets these in the first place
                    if (dy.abs() <= self.termy) {
                        println!("vertical overlap vector: {}, {}", dx, dy);
                        self.sprite.y -= dy;
                        if dy > 0.0 {
                            self.grounded = true;
                            self.vely = 0.0;
                        }
                    }
                }
                //then check horizontal collisions
                let mut horizontal: HitBox = self.get_hitbox();
                horizontal.y += 2.0;
                horizontal.h -= self.termy + 2.0;
                //println!("horizontal hitbox: {},{},{},{}", horizontal.x - self.sprite.x, horizontal.y - self.sprite.y, horizontal.w, horizontal.h);
                if hitbox::box_collision(other, horizontal) {
                    let (dx, dy) = hitbox::box_overlap(other, horizontal);
                    if (dx.abs() <= self.termx) {
                        println!("horizontal overlap vector: {}, {}", dx, dy);
                        self.sprite.x -= dx;
                        if ce.direction.0 < 0.0 && self.velx < 0.0 {
                            self.velx = 0.0;
                        }
                        if ce.direction.0 > 0.0 && self.velx > 0.0 {
                            self.velx = 0.0;
                        }
                    }
                }
            }
            _ => {}
        }
    }*/
    fn collision(&self, other: HitBox) -> CollisionEvent {
        return CollisionEvent {
            direction: (0.0, 0.0),
            impact: -1,
        };
    }
}

impl Entity for Murph {
    fn new() -> Self {
        let mut m = Murph {
            health: 255,
            sprite: Sprite::new(data::MURPH),
            velx: 0.0,
            vely: 0.0,
            termx: 8.0,
            termy: 8.0,
            blocked_left: false,
            blocked_right: false,
            blocked_above: false,
            blocked_ground: false,
            difference_h: 0.0,
            difference_v: 0.0,
        };
        m.sprite.w = 16.0;
        m.sprite.h = 16.0;
        return m;
    }
    fn tick(&mut self, gs: &GameState) {
        println!{"{}, {}", self.velx, self.vely};
        if self.health <= 0 {
            println!("I'm dead!");
        }
        if gs.primary {
            self.sprite.flip_v = true;
        } else {
            self.sprite.flip_v = false;
        }
        /*if gs.up {
            self.vely = -self.termy;
        }
        if gs.down {
            self.vely = self.termy;
        }
        if gs.left {
            self.velx = -self.termx;
        }
        if gs.right {
            self.velx = self.termx;
        }*/
        if gs.up {
            if self.blocked_ground {
                self.vely = -8.0;
            }
        }
        if gs.left && !gs.right {
            self.velx -= 0.2;
            self.sprite.anim_index += 0.25;
            self.sprite.flip_h = true;
        } else if gs.right && !gs.left {
            self.velx += 0.2;
            self.sprite.anim_index += 0.25;
            self.sprite.flip_h = false;
        } else {
            //if he's on the gound then apply friction
            if self.blocked_ground {
                self.velx /= 2.0;
            } else {
                self.velx /= 1.2;
            }
            //set a lower limit to how slow he can go before stopping
            if self.velx < 0.02 && self.velx > 0.0 {
                self.velx = 0.0;
            }
            if self.velx > -0.02 && self.velx < 0.0 {
                self.velx = 0.0;
            }
            //if not moving, don't animate
            self.sprite.anim_index = 0.0;
        }

        //gravity
        if !self.blocked_ground {
            //println!("gravity applies!");
            self.vely += 0.5;
        }

        //terminal velocities
        if self.vely > self.termy {
            self.vely = self.termy;
        }
        if self.velx > self.termx {
            self.velx = self.termx;
        }
        if self.velx < -self.termx {
            self.velx = -self.termx;
        }

        //if you can go, then go
        if (self.velx < 0.0 && !self.blocked_left) || (self.velx > 0.0 && !self.blocked_right) {
            self.sprite.x += self.velx;
        } else if self.difference_v == 0.0 {
            //otherwise make up the rest
            self.velx = 0.0;
            self.sprite.x += self.difference_h;
        }
        if (self.vely < 0.0 && !self.blocked_above) || (self.vely > 0.0 && !self.blocked_ground) {
            self.sprite.y += self.vely;
        } else {
            self.vely = 0.0;
            self.sprite.y += self.difference_v;
        }

        //reset collision
        self.blocked_left = false;
        self.blocked_right = false;
        self.blocked_above = false;
        self.blocked_ground = false;
        self.difference_h = 0.0;
        self.difference_v = 0.0;
    }
}
