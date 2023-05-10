use super::core_types::TurnData;
use super::core_types::TurnApplyResult;

pub struct GameLogic {
    
}

impl GameLogic {
    pub fn apply_turn(&mut self, r: TurnData) -> TurnApplyResult {
        TurnApplyResult::Valid
    }    
}