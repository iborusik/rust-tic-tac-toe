use std::collections::LinkedList;

use super::core_types::TurnData;
use super::core_types::TurnApplyResult;
use super::core_types::Cells;
use super::core_types::Cell;

pub struct GameLogic {
    
}

enum ECellDiag {
    Vertical,
    Horizontal,
    Positive,
    Negative
}

impl GameLogic {
    
    fn get_cell_diagonal<'a>(&self, cells: &'a Cells, diag: ECellDiag, i: u32, j: u32, rows: u32, colls: u32) -> Option<LinkedList<&'a Cell>> {
        let mut l : LinkedList<&Cell> = LinkedList::new();
        
        match diag {
            ECellDiag::Vertical => {
                if i == 0 || i == rows - 1 {
                    return None;
                }                                
                let up = i - 1;
                let dn = i + 1;
                for i in up..dn {
                    l.push_back(&cells[(i*colls + j) as usize]);
                }
            }
            
            ECellDiag::Horizontal => {
                if j == 0 || j == rows - 1 {
                    return None;
                }                                
                let up = j - 1;
                let dn = j + 1;
                for i in up..dn {
                    l.push_back(&cells[(i*colls + j) as usize]);
                }                
            }
            
            ECellDiag::Positive => {
                todo!();
            }
            
            ECellDiag::Negative => {
                todo!();
            }       
        }

        if l.is_empty() {
            return None;
        }
        return Some(l);
    }
    
    fn cell_at_index<'a>(&self, cells: &'a Cells, i: u32, j: u32, colls: u32) -> &'a Cell {
        let position : usize = (i * colls + j) as usize;
        &cells[position]
    }
    
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

    pub fn check_win(&self, grid: &Cells, player_color: u32, rows: u32, colls: u32, extra: Option<(u32, u32)>) -> bool {

        let check_extra = |i: u32, j: u32| -> Option<u32> {
            let ret: Option<u32> = match extra {
                None => Option::<u32>::None,
                Some((x ,y)) => {
                    let mut r = Option::<u32>::None;
                    if x == i && j == y {
                        r = Some(player_color)                        
                    }                    
                    r
                } 
            };            
            ret
        };
        
        let check_list_win = |cells: &Option<LinkedList<&Cell>>, color:u32| -> bool {
            match &cells {
                None => false,
                &Some(x) => {
                    let res = x.iter().all(|cell| {
                        match cell._color {
                            None => {
                                let extra = check_extra(cell._i, cell._j);
                                if let Some(x) = extra {
                                    return x == color;
                                } else {
                                    return false;
                                }                                
                            },
                            
                            Some(x) => {
                                return x == color;
                            }
                        }            
                    });
                    return res;
                }
            }
        };
        
        let mut win = false;        
        for i in 0 .. rows {
            for j in 0 .. colls {                
                // vert
                let diag: Option<LinkedList<&Cell>> = self.get_cell_diagonal(grid, ECellDiag::Vertical, i, j, rows, colls);
                win |= check_list_win(&diag, player_color);
                
                // hor
                let diag: Option<LinkedList<&Cell>> = self.get_cell_diagonal(grid, ECellDiag::Horizontal, i, j, rows, colls);
                win |= check_list_win(&diag, player_color);
                
                // positive diag
                //let diag: Option<LinkedList<&Cell>> = self.get_cell_diagonal(grid, ECellDiag::Negative, i, j, rows, colls);
                //win |= check_list_win(&diag, player_color);
                
                // negative diag                
                //let diag: Option<LinkedList<&Cell>> = self.get_cell_diagonal(grid, ECellDiag::Positive, i, j, rows, colls);                
                //win |= check_list_win(&diag, player_color);
                
                if win {
                    break;
                }          
            }
        }
        
        win
    }
        
}