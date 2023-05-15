use crate::core;

pub struct ConsoleInput {
}

impl core::Input for ConsoleInput {
    fn read_input(&self) -> core::InputResult {        
        let input = |prompt: &str | -> i32 {
            let mut parsed;            
            loop {
                println!("{}", prompt);
                let mut line = String::new();            
                std::io::stdin().read_line(&mut line).unwrap();
                let _rr = line.parse::<i32>();
                line.remove(line.len() - 1);
                parsed = line.parse::<i32>().unwrap_or(-1);              
                parsed -= 1;  
                if parsed >= 0 {
                    break;
                }
            }
            return parsed;
        };        
        
        core::InputResult {
            i: input("Enter row:") as u32,
            j: input("Enter coll:") as u32
        }
    }
}