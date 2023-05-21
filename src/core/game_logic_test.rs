
use super::common_test;
use super::game;
use super::game_logic;
use super::core_types::Cell;

#[test]
fn tst_cell_diagonal() {
    let rows = 3;
    let colls = 3;
            
    let grid = common_test::tests::create_grid(rows, colls);
    {
        let diag = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 0, rows, colls);    
        assert!(diag.is_none(), "diag failed");
        
        let diag = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 1, rows, colls);    
        assert!(diag.is_some(), "opps");        
        
        let cells = diag.unwrap();
        let mut b = cells.iter();
        let c1 = b.next().unwrap();
        let c2 = b.next().unwrap();
        let c3 = b.next().unwrap();
    
        assert!(cells.len() == 3, "opps");
        assert!(c1._i == 0 && c1._j == 0);
        assert!(c2._i == 0 && c2._j == 1);
        assert!(c3._i == 0 && c3._j == 2);
        
        
        let diag: Option<std::collections::LinkedList<&Cell>> = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , rows - 1, colls - 2, rows, colls);    
        assert!(diag.is_some(), "opps");        
        
        let cells = diag.unwrap();
        let mut b = cells.iter();
        let c1 = b.next().unwrap();
        let c2 = b.next().unwrap();
        let c3 = b.next().unwrap();
    
        assert!(cells.len() == 3, "opps");
        assert!(c1._i == rows - 1 && c1._j == 0);
        assert!(c2._i == rows - 1 && c2._j == 1);
        assert!(c3._i == rows - 1 && c3._j == 2);
    }
}

fn clear_gread(cells: &mut Vec<Cell>) {
    for i in cells {
        i._color = None;
    }
}
fn set_cell_color(cells:& mut Vec<Cell>, i: u32, j: u32, color: u32, colls: u32)
{
    let cell = game_logic::cell_at_index(cells, i, j, colls); 
    cell._color = Some(color);
    
}

#[test]
fn test_win_lines()
{
    let rows = 3;
    let colls = 3;
            
    let mut grid = common_test::tests::create_grid(rows, colls); 
    
    clear_gread(&mut grid);
    
    let color: u32 = 1;
    set_cell_color(&mut grid, 0, 0, color, colls);
    set_cell_color(&mut grid, 0, 1, color, colls);
    
    {            
        let diag: Option<std::collections::LinkedList<&Cell>> = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 1, rows, colls); 
        let win_line = game_logic::check_line_win(&diag, color, None);
        assert!(win_line == false);
        let win_line = game_logic::check_line_win(&diag, color, Some((0, 2)));
        assert!(win_line == true);        
    }
 
    {
        set_cell_color(&mut grid, 0, 2, color, colls);        
        let diag: Option<std::collections::LinkedList<&Cell>> = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 1, rows, colls);           
        let win_line = game_logic::check_line_win(&diag, color, None);
        assert!(win_line == true)
    }
}
