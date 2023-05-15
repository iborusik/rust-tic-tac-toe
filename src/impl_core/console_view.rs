use crate::core;

pub struct ConsoleView {
}

impl core::View for ConsoleView {
    fn draw(&self, game: &core::Game) {
        println!("--------------------------------");
        print!(" |");
        for j in 0 .. game.colls() {   
            print!("{} ", j + 1);
        }
        
        println!();
                         
        for i in 0 .. game.rows() {
            print!("{}|", i + 1);
            for j in 0 .. game.colls() {                
                let c: char = match  game.cells()[(i*game.colls() + j) as usize]._color {
                    None => '.',
                    Some(x) => {
                        let mut r = 'x';
                        if x == 0 {
                            r = '0';
                        }
                        r
                    }
                };
                print!("{} ", c);
            }
            println!("");
        }
        println!("--------------------------------");        
    }
}