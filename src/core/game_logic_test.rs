
use super::core_types::Cell;
use super::core_types;
use super::common_test;
use super::game_logic;

#[test]
fn check_cell() {
    let rows = 3;
    let colls = 3;
    let grid = common_test::tests::create_grid(10, 10);
    {
        let diag = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 0, rows, colls);    
        assert!(diag.is_none(), "diag failed");
        
        let diag = game_logic::get_cell_diagonal(&grid, game_logic::ECellDiag::Horizontal , 0, 2, rows, colls);    
        assert!(diag.is_some(), "opps");
        assert!(diag.unwrap().len() == 3, "opps");
    }
    
}
