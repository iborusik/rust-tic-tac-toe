use super::core_types::TurnData;
use super::core_types::TurnApplyResult;
use super::core_types::Cells;
use super::core_types::Cell;

pub struct GameLogic {
    
}

impl GameLogic {
    pub fn apply_turn(&mut self, r: TurnData, cells: &mut Cells, rows: u32, colls: u32) -> TurnApplyResult {
        if r.i >= rows || r.j >= colls {
            return TurnApplyResult::NoValid;
        }
        let cell = &mut cells[(r.i * colls + r.j) as usize];
        
        match cell._color {
            Some(x) => {
                println!("player {} in cell", x);
                return TurnApplyResult::NoValid;
            }
            
            None => {
                cell._color = Some(r.player_index)
            }
        }
                      
        TurnApplyResult::Valid
    } 
}