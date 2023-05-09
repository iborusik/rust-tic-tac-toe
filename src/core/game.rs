
use super::states;
use super::player;
use super::input;
use super::view;

use std::cmp;
struct Cell {
    _i: u32,
    _j: u32
}

pub struct Game {
    // private
    state   : Option<Box<dyn states::State>>,
    
    _colls  : u32,
    _rows   : u32,
    _box    : Vec<Cell>,
    _p_index: u32,
    _players: Vec<Box<dyn player::Player>>,
    _view   : Box<dyn view::View>,
    _input  : Box<dyn input::Input>
}

impl Game {    
    pub fn init(&mut self) {        
        (0..self._colls).for_each(|i|{
            (0..self._rows).for_each(|j|{
                self._box.push(Cell {
                    _i: i,
                    _j: j
                });
            });
        });
    }
    
    pub fn update(&mut self) -> bool {            
        if let Some(s) = self.state.take() {
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
            _players: vec![
                Box::new(player::Human{}),
                Box::new(player::Bot{})
            ],
            _view: view,
            _input: input
        };
        g.init();
        return g;
    }
    
    pub fn player_turn(&mut self) {
        println!("player{} turn", self._p_index);
        self._players[self._p_index as usize].turn();
        self._p_index += 1;
        if self._p_index == 2 {
            self._p_index = 0;
        }
    }
    
    pub fn is_game_over(&self) -> bool {
        return false;
    }
    
    pub fn draw(&self) {
        
    }
    
}