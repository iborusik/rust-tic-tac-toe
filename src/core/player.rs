use super::Game;
use super::core_types;

pub trait Player {
    fn turn(&self, game: &Game) -> core_types::TurnData;
}

pub struct Human { }

pub struct Bot { }

impl Player for Human {
    fn turn(&self, game: &Game) -> core_types::TurnData {
        println!("human turn");
        let res = game.input().read_input();
        core_types::TurnData {
            i: res.i,
            j: res.j,
            player_index : 0
        }                
    }
}

impl Player for Bot {
    fn turn(&self, _game: &Game)  -> core_types::TurnData {
        println!("bot turn");
        core_types::TurnData {
            i: 0,
            j: 0,
            player_index: 0
        }
    }
}