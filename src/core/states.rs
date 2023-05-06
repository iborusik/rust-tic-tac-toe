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

pub struct PlayerTurn {
}

pub struct GameOver {
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
        return Box::new(PlayerTurn{});
    }
}

impl State for Idle {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("idle...");
        thread::sleep(time::Duration::from_secs(1));            
        return self;
    }
}

impl State for  PlayerTurn {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("PlayerTurn");  
        game.player_turn();
        thread::sleep(time::Duration::from_secs(1));
        if game.is_game_over() {
            return Box::new(GameOver{});
        }   
        return Box::new(DisplayField{});
    }    
}

impl State for GameOver {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("GameOver");  
        thread::sleep(time::Duration::from_secs(1));         
        return Box::new(Idle{});
    }       
}