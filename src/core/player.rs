use crate::core::game_logic;

use super::Game;
use super::core_types;
use super::bot_logic;

pub trait Player {
    fn turn(&self, game: &Game) -> core_types::TurnData;
}

struct PlayerInfo {
    color: u32
}

pub struct Human { 
    info: PlayerInfo        
}

impl Human {
    pub fn new(color: u32) -> Human {
        Human { info: PlayerInfo { color: color } }
    }
}


impl Bot {
    pub fn new(color: u32) -> Bot {
        Bot { info: PlayerInfo { color: color } }
    }
}

pub struct Bot { 
    info: PlayerInfo
}

impl Player for Human {
    fn turn(&self, game: &Game) -> core_types::TurnData {
        println!("human turn");
        let res = game.input().read_move();
        core_types::TurnData {
            i: res.i - 1,
            j: res.j - 1,
            player_index : 0
        }                
    }
}

impl Player for Bot {
    fn turn(&self, _game: &Game)  -> core_types::TurnData {
        println!("bot turn");       
        let next_turn = bot_logic::find_next_move(_game.cells(), self.info.color,
            _game.rows(), _game.colls());
        let result = next_turn.unwrap();
        core_types::TurnData {
            i: result.0,
            j: result.1,
            player_index: 0
        }
    }
}