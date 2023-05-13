use crate::core::core_types::TurnApplyResult;

use super::Game;
use std::{thread, time};
use super::core_types;

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
        game.draw();
        return Box::new(PlayerTurn{});
    }
}

impl State for Idle {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        thread::sleep(time::Duration::from_secs(1));            
        return self;
    }
}

impl State for  PlayerTurn {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        let result = game.player_turn();
        match result {
            TurnApplyResult::Valid => {
                if game.is_game_over() {
                    return Box::new(GameOver{});
                }                
            }
                        
            TurnApplyResult::NoValid => {
                print!("not valid move");
                return Box::new(PlayerTurn{});
            }            
        }
        
        Box::new(DisplayField{})
    }    
}

impl State for GameOver {
    fn update(self: Box<Self>, game: &mut Game) -> Box<dyn State> {
        println!("GameOver");  
        thread::sleep(time::Duration::from_secs(1));         
        return Box::new(Idle{});
    }       
}