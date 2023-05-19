pub mod tests {
    
    use super::super::core_types::Cell;
    use super::super::core_types::Cells;
    
    pub fn create_grid(rows: u32, colls: u32) -> Cells {
        let mut c: Cells = Cells::new();
        (0..rows).for_each(|i|{
            (0..colls).for_each(|j|{
                    c.push(Cell {
                        _i: i,
                        _j: j,
                        _color: None
                    });
                });
            });
        return c;
    }
}