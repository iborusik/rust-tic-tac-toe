mod core;

fn main() {
    let mut game : core::Game = core::Game::new();
    
    while game.update() {
    }
}
