use super::Game;
use std::{thread, time};

pub trait State {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State>;
    
    fn is_game_over(self: Box<Self>) -> bool {
        return false;
    }
}

pub struct Idle {    
}

pub struct Intro {    
}

pub struct DisplayField {    
}

impl State for Intro {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("Welcome to tic-tac-toe rust demo.");
        return Box::new(DisplayField{});
    }
}

impl State for DisplayField {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("DrawField");
        return Box::new(Idle{});
    }
}

impl State for Idle {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("idle...");
        let ten_millis = time::Duration::from_secs(1);
        thread::sleep(ten_millis);              
        return self;
    }
}