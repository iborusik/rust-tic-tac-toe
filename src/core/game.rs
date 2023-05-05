
use super::states;

pub struct Game {
        
    // private
    state: Option<Box<dyn states::State>>
}

impl Game {
    pub fn init(&mut self) {
        
    }
    
    pub fn update(&mut self) -> bool {            
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(self));
        }        
        return true;
    }
    
    pub fn new() -> Game {
        Game {
            state: Some(Box::new (states::Intro{}))
        }
    }        
}