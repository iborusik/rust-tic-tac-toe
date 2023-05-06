mod core;

fn main() {
    let mut game : core::Game = core::Game::new(10, 20);
    
    while game.update() 
    {}
}
