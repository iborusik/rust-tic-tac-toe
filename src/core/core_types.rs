pub struct TurnData {
    pub i: u32,
    pub j: u32,
    pub player_index: u32
}

pub enum TurnApplyResult {
    Valid,
    NoValid
}

pub struct Cell {
    pub _i: u32,
    pub _j: u32,
    pub _color: Option<u32>
}

pub enum PlayerType {
    EHuman,
    EBot
}

pub type Cells = Vec<Cell>;