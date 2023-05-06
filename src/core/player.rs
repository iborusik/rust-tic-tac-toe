
pub trait Player {
    fn turn(&mut self);
}

pub struct Human {
}

pub struct Bot {
}

impl Player for Human {
    fn turn(&mut self) {
        println!("human turn");        
    }
}

impl Player for Bot {
    fn turn(&mut self) {
        println!("bot turn");
    }
}