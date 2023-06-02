
use crate::core::core_types::TurnApplyResult;
use crate::core::core_types::TurnData;
use crate::core::player::Player;

use super::core_types::PlayerType;
use super::game_logic;
use super::states;
use super::player;
use super::input;
use super::view;
use super::core_types::Cells;
use super::core_types::Cell;

pub struct Game {
    
    // private
    state   : Option<Box<dyn states::State>>,
    
    _colls  : u32,
    _rows   : u32,
    _box    : Cells,
    _p_index: u32,
    _players: Vec<Box<dyn player::Player>>,
    _view   : Box<dyn view::View>,
    _input  : Box<dyn input::Input>,
    _win    : Option<u32>
}

impl Game {    
    pub fn init(&mut self) {        
        (0..self._colls).for_each(|i|{
            (0..self._rows).for_each(|j|{
                self._box.push(Cell {
                    _i: i,
                    _j: j,
                    _color: None
                });
            });
        });
    }
    
    pub fn update(&mut self) -> bool {            
        if let Some(s)= self.state.take() {
            self.state = Some(s.update(self));
        }        
        return true;
    }
    
    pub fn new(colls: u32, rows: u32, view: Box<dyn view::View>, input: Box<dyn input::Input>) -> Game {
        let mut g = Game {
            state: Some(Box::new (states::Intro{})),
            _colls  : colls,
            _rows   : rows,
            _box    : Vec::new(),
            _p_index: 0,
            _players: vec![],
            _view: view,
            _input: input,           
            _win: None
        };
        g.init();
        return g;
    }
    
    pub fn add_players(&mut self, players: Vec<PlayerType>) {
        let mut ind: i32 = -1;
        self._players = players.into_iter().map(|t| -> Box<dyn Player> {
           ind += 1;
           match t {
               PlayerType::EHuman => Box::new(player::Human::new(ind as u32)),
               PlayerType::EBot => Box::new(player::Bot::new(ind as u32))
           }            
        }).collect();
    }
    
    pub fn player_turn(&mut self) -> TurnApplyResult {
        println!("player: {} -> turn", match self._p_index {1 => "x", _ => "0"});
        let player: &Box<dyn Player> = &self._players[self._p_index as usize];
        
        let mut turn_result: TurnData = player.turn(self);
        turn_result.player_index = self._p_index;
        
        let result: TurnApplyResult = game_logic::apply_turn(turn_result, 
            &mut self._box, 
            self._rows, 
            self._colls);        
        
        let win = game_logic::check_win(&self._box, 
            self._p_index,
            self._rows,
            self._colls,
            None);
            
        if win {
            self._win = Some(self._p_index);
        }
            
        match result {
            TurnApplyResult::Valid => {
                self._p_index += 1;
                if self._p_index == 2 {
                    self._p_index = 0;
                }                
            },
            TurnApplyResult::NoValid => {
                print!("no valid move");
            } 
        }

        result
    }
    
    pub fn is_game_over(&self) -> bool {        
        return self._win.is_some();
    }
    
    pub fn draw(&self) {       
        self._view.draw(self);
    }
    
    pub fn input(&self) -> &Box<dyn input::Input> {
        return  &self._input;
    }
    
    pub fn rows(&self) -> u32 {
        self._rows
    }
    
    pub fn colls(&self) -> u32 {
        self._colls
    }
    
    pub fn cells(&self) -> &Cells {
        return &self._box;
    } 
    
    pub fn get_win_index(&self) -> u32 {
         return self._win.unwrap();
    }
    
}