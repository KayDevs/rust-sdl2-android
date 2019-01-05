extern crate sdl2;

#[derive(Clone, Copy)]
pub struct HitBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy)]
pub struct CollisionEvent {
    //whether or not you collided
    //collided: bool,
    //direction from which the hit came
    pub direction: (f32, f32),
    //how strong the hit is
    pub impact: i32,
    /*
    -this system allows collision to work the same exact way for enemies, items, platforms, etc. 
    -enemies, for example, could simulate damage by having an ultra-high impact
    -walls and such would have zero impact, but have a direction pushing characters outward the appropriate amount.
    -physics items would have both - if they're going fast enough they could deal damage, but not necessarily.
    -the direction/impact can also be used as metadata for other uses, depending on how the object handles it.
    -(as per my design, -impact = object class)
     */
}

/*
-player:
--direction: (direction)
--impact: -1
-enemy:
--direction: (0, 0)
--impact: 255
-wall/platform:
--direction: (collision response data)
--impact: 0
-other items:
--direction: (0, 0)
--impact: -(item id)
*/

pub trait Collide {
    //simple box collision for now
    //but also maybe open it up to radiuses, or polygonal geometry
    fn get_hitbox(&self) -> HitBox;
    //how their hits should register with themselves
    fn handle_collision(&mut self, CollisionEvent, other: HitBox);
    //how their hits should register with other objects
    fn collision(&self, other: HitBox) -> CollisionEvent; 
}

pub fn box_collision(a: HitBox, b: HitBox) -> bool {
    //TODO: insert actual collision detection code here.
    return a.x < b.x + b.w
        && a.x + a.w  > b.x
        && a.y < b.y + b.h
        && a.y + a.h  > b.y;
}

//a: pusher; b: pushed
pub fn box_overlap(a: HitBox, b: HitBox) -> (f32, f32) {
    let overlap_x: f32;
    if b.x < a.x {
        overlap_x = (b.x + b.w) - a.x;
    } else {
        overlap_x = b.x - (a.x + a.w);
    }
    let overlap_y: f32;
    if b.y < a.y {
        overlap_y = (b.y + b.h) - a.y;
    } else {
        overlap_y = b.y - (a.y + a.h);
    }
    return (overlap_x, overlap_y);
}
pub fn box_mtv(a: HitBox, b: HitBox) -> (f32, f32) {
    let (overlap_x, overlap_y) = box_overlap(a, b);
    if overlap_y.abs() > overlap_x.abs() {
        return (overlap_x, 0.0);
    } else {
        return (0.0, overlap_y);
    }
}
