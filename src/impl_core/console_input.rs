use crate::core;
use crate::core::PlayerType;

type InputLimit = (Option<u32>, Option<u32>);


pub struct ConsoleInput {
}

impl ConsoleInput {
    pub fn read_integer(&self, prompt: &str,
            limits: InputLimit) -> u32 {
        let mut parsed: u32 = 0;
        loop {
            println!("{}", prompt);
            
            let mut line = String::new();            
            std::io::stdin().read_line(&mut line).unwrap();
            let _rr = line.parse::<u32>();
            line.remove(line.len() - 1);
            let parsed_val = line.parse::<u32>();
                        
            parsed = parsed_val.clone().unwrap_or(0);
            
            //check limits        
            let mut pass_limit = parsed_val.is_ok();                        
            pass_limit |= limits.0.is_none() || parsed >= *limits.0.as_ref().unwrap();
            pass_limit |= limits.1.is_none() || parsed <= *limits.1.as_ref().unwrap();
            
            if pass_limit {
                break;
            }
        }
        return parsed;
    }        
}

impl core::Input for ConsoleInput {
    
    fn read_move(&self) -> core::InputMoveResult {        
        core::InputMoveResult {
            i: ConsoleInput::read_integer(&self,"Enter row:", (None, None)) as u32,
            j: ConsoleInput::read_integer(&self,"Enter coll:", (None, None)) as u32
        }
    }
    
    fn read_player(&self, player_index: u32) -> core::PlayerType {
        let s = format!("Choose player {} (1-human, 2-bot)", player_index);
        let ref_f = s.as_str();
        let result = ConsoleInput::read_integer(&self, 
            ref_f, 
                (Some(1), Some(2)));
        
        match result {
            1 => PlayerType::EHuman,
            2 => PlayerType:: EBot,
            _ => panic!("unexpected result")
        }
    }
}