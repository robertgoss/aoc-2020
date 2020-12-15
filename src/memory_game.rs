
use std::collections::HashMap;

pub struct Game {
    initial : Vec<u64>,
    index : u64,
    memory : HashMap<u64, u64>,
    previous : u64
}

impl Game {
    pub fn new(initial : Vec<u64>) -> Game {
        Game {
            initial : initial,
            index : 0,
            memory : HashMap::new(),
            previous : 0
        }
    }

    fn next_num(self : &Self) -> u64 {
        *self.initial.get(self.index as usize).unwrap_or(
            &self.memory.get(&self.previous).map(
                |prev_index| self.index - prev_index
            ).unwrap_or(0)
        )
    }
}

impl Iterator for Game {
    type Item = u64;

    fn next(self : &mut Self) -> Option<u64> {
        let num = self.next_num();
        // Update state
        self.memory.insert(self.previous, self.index);
        self.previous = num;
        self.index += 1;
        Some(num)
    }
}
