mod core;
mod impl_core;

fn main() {       
    let console_view: Box<impl_core::ConsoleView>  = Box::new(impl_core::ConsoleView{});
    let console_input: Box<impl_core::ConsoleInput> = Box::new(impl_core::ConsoleInput{});
    
    let mut game : core::Game = core::Game::new(5,5, console_view, console_input);        
    while game.update() { }
}
