use crate::core;

pub struct ConsoleView {
}

impl core::View for ConsoleView {
    fn draw(&self, game: &core::Game) {
        println!("--------------------------------");
        for i in 0 .. game.rows() {
            for j in 0 .. game.colls() {
                let c: char;
                match  game.cells()[(i*game.colls() + j) as usize]._color {
                    None => c = '0',
                    Some(x) => {
                        c = std::char::from_digit(x + 1, 10).unwrap();
                    }
                };
                print!("{} ", c);
            }
            println!("");
        }
        println!("--------------------------------");        
    }
}