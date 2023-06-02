use super::core_types::Cells;
use super::core_types::Cell;
use super::game_logic;

/**
 * Find best valid move for the player with color
 */
pub fn find_next_move(cells: &Cells, color: u32, 
    rows: u32, colls: u32) -> Option<(u32, u32)> {
    
    let opponent_color: u32 = match color {
        0 => 1,
        _ => 0            
    };
    // 1. check if current player can finish in one move
    // it should do it
    for l in cells.iter() {
        if l._color.is_some() {
            continue;
        }
        
        let win = game_logic::check_win(cells, color, rows, colls, Some((l._i, l._j)));
        if win {
            return Some((l._i, l._j));
        }
    }
        
    // 2. check for the opponent. 
    // if we he can finish game with one move
    // we need to block it    
    for l in cells.iter() {
        if l._color.is_some() {
            continue;
        }
        
        let win = game_logic::check_win(cells, opponent_color, rows, colls, Some((l._i, l._j)));
        if win {
            return Some((l._i, l._j));
        }
    }
    
    // 3. TMP: todo: improve this
    // put first free cell
    for l in cells.iter() {
        if l._color.is_some() {
            continue;
        }                
        return Some((l._i, l._j));        
    }
        
    
    
    return None;
}
