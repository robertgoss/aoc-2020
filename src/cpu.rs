use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum Instruction {
    NOP,
    ACC,
    JMP
}

pub struct Program {
    instructions : Vec<(Instruction, i64)>
}

pub struct ProgramState<'a> {
    counter : usize,
    override_instruction : &'a HashMap<usize, Instruction>,
    accumalator : i64,
    instructions : HashSet<usize>,
    program : &'a Program
}

impl Instruction {
    fn parse_type(string : &str) -> Option<Instruction> {
        match string {
            "nop" => Some(Instruction::NOP),
            "acc" => Some(Instruction::ACC),
            "jmp" => Some(Instruction::JMP),
            _ => None
        }
    }
    fn flip(self : &Self) -> Instruction {
        match self {
            Instruction::NOP => Instruction::JMP,
            Instruction::JMP => Instruction::NOP,
            Instruction::ACC => Instruction::ACC
        }
    }
}
fn parse_line(line : &str) -> Option<(Instruction, i64)> {
    let parsed = line.split_once(" ").map(
        |(in_type, int)| (Instruction::parse_type(in_type), int.parse::<i64>().ok())
    );
    match parsed {
        Some((Some(in_type), Some(int))) => Some( (in_type, int) ),
        _ => None
    }
}
impl Program {
    pub fn from_lines<I>(lines : I) -> Program 
      where I : Iterator<Item = String> 
    {
        Program {
            instructions : lines.filter_map(
                |line| parse_line(&line)
            ).collect()
        }
    }
    fn run_override(self : &Self, overridden : &HashMap<usize, Instruction>) -> (bool, i64) {
        let mut state = ProgramState::new(self, overridden);
        let mut acc = 0;
        while !state.looped() && !state.at_end() {
            state.run_once();
            acc = state.accumalator;
        }
        (state.at_end(), acc)
    }
    pub fn run(self : &Self) -> i64 {
        let dummy : HashMap<usize, Instruction> = HashMap::new();
        let (_, acc) = self.run_override(&dummy);
        acc
    }
    fn run_altered(self : &Self, flipped_index : usize) -> Option<i64> {
        let mut flip : HashMap<usize, Instruction> = HashMap::new();
        let (current, _) = self.instructions[flipped_index];
        flip.insert(flipped_index, current.flip());
        let (success, acc) = self.run_override(&flip);
        if success {
            Some(acc)
        } else {
            None
        }
    }
    pub fn fix(self : &Self) -> Option<i64> {
        (0..self.instructions.len()).filter_map(
            |index| self.run_altered(index)
        ).next()
    }
    
}
impl<'a> ProgramState<'a> {
    fn new(
        program : &'a Program, 
        override_instruction : &'a HashMap<usize, Instruction>
    ) -> ProgramState<'a> {
        ProgramState {
            program : program,
            counter : 0,
            override_instruction : override_instruction,
            accumalator : 0,
            instructions : HashSet::new()
        }
    }
    fn looped(self : &Self) -> bool {
        self.instructions.contains(&self.counter)
    }
    fn at_end(self : &Self) -> bool {
        self.counter == self.program.instructions.len()
    }
    fn run_once(self : &mut Self) {
        self.instructions.insert(self.counter);
        let (initial_instruction, delta) = self.program.instructions[self.counter];
        let instruction = *self.override_instruction.get(
            &self.counter
        ).unwrap_or(&initial_instruction);
        self.run_instruction((instruction, delta))
    }
    fn run_instruction(self : &mut Self, instruction : (Instruction, i64)) {
        match instruction {
            (Instruction::NOP, _) => {
                self.counter += 1;
            },
            (Instruction::ACC, delta) => {
                self.accumalator += delta;
                self.counter += 1;
            },
            (Instruction::JMP, delta) => {
                self.counter = ((self.counter as i64) + delta) as usize;
            }
        }
    }
}