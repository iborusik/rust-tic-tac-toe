mod core;
mod impl_core;

fn main() {
    let console_view = Box::new(impl_core::ConsoleView{});
    let console_input = Box::new(impl_core::ConsoleInput{});
    let mut game : core::Game = core::Game::new(10,20, console_view, console_input);
    
    while game.update() 
    {
        game.draw();
    }
}
