
use super::states;

struct Cell {
    _i: u32,
    _j: u32
}

pub struct Game {
    
    // private
    state   : Option<Box<dyn states::State>>,
    
    _colls  : u32,
    _rows   : u32,
    _box    : Vec<Cell>
}

impl Game {    
    pub fn init(&mut self) {
        // init filed data
        let convert =  |index| -> Cell {
            let i: u32 = index / self._colls;
            let j: u32 = index - i * self._colls;
            return Cell {
                _i: i,
                _j: j
            };
        };

        let it = (0..(self._colls*self._rows)).map(convert);
        self._box = it.collect();
    }
    
    pub fn update(&mut self) -> bool {            
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(self));
        }        
        return true;
    }
    
    pub fn new(colls: u32, rows: u32) -> Game {
        let mut g = Game {
            state: Some(Box::new (states::Intro{})),
            _colls  : colls,
            _rows   : rows,
            _box: Vec::new()
        };
        g.init();
        return g;
    }        
}