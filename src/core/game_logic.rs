use std::collections::LinkedList;

use super::core_types::TurnData;
use super::core_types::TurnApplyResult;
use super::core_types::Cells;
use super::core_types::Cell;

pub enum ECellDiag {
    Vertical,
    Horizontal,
    Positive,
    Negative
}

pub fn get_cell_diagonal<'a>(cells: &'a Cells, diag: ECellDiag, i: u32, j: u32, rows: u32, colls: u32) -> Option<LinkedList<&'a Cell>> {
    let mut l : LinkedList<&Cell> = LinkedList::new();
    
    match diag {
        ECellDiag::Vertical => {
            if i == 0 || i == rows - 1 {
                return None;
            }                                
            let up = i - 1;
            let dn = i + 1;
            for i in up..dn + 1 {
                l.push_back(&cells[(i*colls + j) as usize]);
            }
        }
        
        ECellDiag::Horizontal => {
            if j == 0 || j == colls - 1 {
                return None;
            }                                
            let left = j - 1;
            let right = j + 1;
            for j in left..right + 1 {
                l.push_back(&cells[(i*colls + j) as usize]);
            }                
        }
        
        ECellDiag::Positive => {
            if j == 0 || j == colls - 1 {
                return None;
            }
            if i == 0 || i == rows - 1 {
                return None;
            }
            let left = j - 1;
            let right = j + 1;
            let mut bottom = i + 1;
            for j in left..right + 1 {
                l.push_back(&cells[(bottom*colls + j) as usize]);                
                bottom = bottom.saturating_sub(1);
            }   
        }
        
        ECellDiag::Negative => {
            if j == 0 || j == colls - 1 {
                return None;
            }
            if i == 0 || i == rows - 1 {
                return None;
            }
            let left = j - 1;
            let right = j + 1;
            let mut bottom = i - 1;
            for j in left..right + 1 {               
                l.push_back(&cells[(bottom*colls + j) as usize]);
                bottom += 1; 
            }  
        }       
    }

    if l.is_empty() {
        return None;
    }
    return Some(l);
}

pub fn cell_at_index<'a>(cells: &'a mut Cells, i: u32, j: u32, colls: u32) -> &'a mut Cell {
    let position : usize = (i * colls + j) as usize;
    &mut cells[position]
}

pub fn apply_turn(r: TurnData, cells: &mut Cells, rows: u32, colls: u32) -> TurnApplyResult {
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

pub fn check_line_win(cells: &Option<LinkedList<&Cell>>, color: u32, extra: Option<(u32, u32)>) -> bool {
    let check_extra = |i: u32, j: u32| -> Option<u32> {
        let ret: Option<u32> = match extra {
            None => Option::<u32>::None,
            Some((x ,y)) => {
                let mut r = Option::<u32>::None;
                if x == i && j == y {
                    r = Some(color)                        
                }                    
                r
            } 
        };            
        ret
    };
    
    let res: bool = match &cells {
        None => false,
        &Some(x) => {
            let res = x.iter().all(|cell: &&Cell| {
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
    };

    res
}

pub fn check_win(grid: &Cells, player_color: u32, rows: u32, colls: u32, extra: Option<(u32, u32)>) -> bool {
    let mut win = false;        
    for i in 0 .. rows {
        for j in 0 .. colls {                
            // vert
            let diag: Option<LinkedList<&Cell>> = get_cell_diagonal(grid, ECellDiag::Vertical, i, j, rows, colls);
            win |= check_line_win(&diag, player_color, extra);
            
            // hor
            let diag: Option<LinkedList<&Cell>> = get_cell_diagonal(grid, ECellDiag::Horizontal, i, j, rows, colls);
            win |= check_line_win(&diag, player_color, extra);
            
            // positive diag
            let diag: Option<LinkedList<&Cell>> = get_cell_diagonal(grid, ECellDiag::Negative, i, j, rows, colls);
            win |= check_line_win(&diag, player_color, extra);
            
            // negative diag                
            let diag: Option<LinkedList<&Cell>> = get_cell_diagonal(grid, ECellDiag::Positive, i, j, rows, colls);                
            win |= check_line_win(&diag, player_color, extra);
            
            if win {
                break;
            }          
        }
    }
    
    win
}