pub trait Entity {
    fn new() -> Self where Self: Sized;
    fn tick(&mut self, &GameState);
    //fn update(&mut self);
}

pub struct GameState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub primary: bool, //A
    pub secondary: bool, //B
}
