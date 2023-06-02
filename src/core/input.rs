use super::core_types::PlayerType;

pub struct InputMoveResult {
    pub i: u32,
    pub j: u32,
}


pub trait Input {
    // todo: rewrite this to async    
    fn read_move(&self) -> InputMoveResult;
    
    fn read_player(&self, player_index: u32) -> PlayerType;
}
