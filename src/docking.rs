use std::collections::HashMap;

#[derive(Clone)]
struct Mask {
    mask : String,
}

fn to_bits(num : &u64, len : usize) -> Vec<bool> {
    (0..len).map(
      |i|  (num >> (len-1-i)) % 2 == 1
    ).collect()
}

fn mask_bit((ch, bit) : (char, &bool)) -> char {
    match ch {
        '0' => '0',
        '1' => '1',
        _ => if *bit { '1' } else { '0' }
    }
}

fn one_mask_bit((ch, bit) : (char, &bool)) -> char {
    match ch {
        '1' => '1',
        'X' => '0',
        _ => if *bit { '1' } else { '0' }
    }
}

fn make_floating(num : u64, index : &[usize]) -> Vec<u64> {
    if index.len() == 0 {
        return vec!(num);
    }
    let mask : u64 = u64::pow(2,index[0] as u32);
    let mut base = make_floating(num, &index[1..]);
    base.extend(make_floating(num + mask, &index[1..]));
    return base;
}

impl Mask {

    pub fn empty() -> Mask {
        Mask {
            mask : "00000000000000000000000000000000".to_string()
        }
    }
    pub fn from_string(string : &str) -> Mask {
        Mask {
            mask : string.to_string()
        }
    }
    pub fn mask(self : &Self, num : &u64) -> u64 {
        let bits = to_bits(num, 36);
        let masked : String 
          = self.mask.chars().zip(bits.iter()).map(mask_bit).collect();
        let res = u64::from_str_radix(&masked, 2).expect("Bad form");
        res
    }

    pub fn decode(self : &Self, num : &u64) -> Vec<u64> {
        let bits = to_bits(num, 36);
        let masked : String
          = self.mask.chars().zip(bits.iter()).map(one_mask_bit).collect();
        let base = u64::from_str_radix(&masked, 2).expect("Bad form");
        let indices : Vec<usize> = self.mask.chars().enumerate().filter(
            |(_,ch)| *ch == 'X'
        ).map(
            |(i,_)| 35 - i
        ).collect();
        make_floating(base, &indices)
    }
}

enum Expression {
    SetMask(Mask),
    AssignVar(u64, u64)
}

impl Expression {
    fn from_line(line : &str) -> Option<Expression> {
        Expression::from_line_mask(line).or(
            Expression::from_line_assign(line)
        )
    }

    fn from_line_mask(line : &str) -> Option<Expression> {
        line.strip_prefix("mask = ").map(
            |rem| Expression::SetMask(Mask::from_string(rem))
        )
    }

    fn from_line_assign(line : &str) -> Option<Expression> {
        line.strip_prefix("mem[").and_then(
            |rem| rem.split_once("] = ").map(
                |(index, val)| Expression::AssignVar(
                    index.parse::<u64>().expect("Badline"),
                    val.parse::<u64>().expect("Badline"),
                )
            )
        )
    }
}

pub struct Program {
    expressions : Vec<Expression>
}

impl Program {
    pub fn from_lines<I>(lines : I) -> Program
      where I : Iterator<Item = String>
    {
        Program {
            expressions : lines.filter_map(
                |line| Expression::from_line(&line)
            ).collect()
        }
    }
}

pub struct Computer {
    current_mask : Mask,
    assignments : HashMap<u64, u64>
}

impl Computer {
    pub fn new() -> Computer 
    {
        Computer {
            current_mask : Mask::empty(),
            assignments : HashMap::new()
        }
    }

    fn run_expression(self: &mut Self, expression : &Expression) {
        match expression {
            Expression::SetMask(mask) => self.current_mask = mask.clone(),
            Expression::AssignVar(index, val) => {
                let masked = self.current_mask.mask(val);
                self.assignments.insert(*index, masked);
            }
        }
    }

    pub fn run(self: &mut Self, program : &Program) {
        for expression in &program.expressions {
            self.run_expression(&expression)
        }
    }

    fn run_decode_expression(self: &mut Self, expression : &Expression) {
        match expression {
            Expression::SetMask(mask) => self.current_mask = mask.clone(),
            Expression::AssignVar(index, val) => {
                let indices = self.current_mask.decode(index);
                for float_index in indices {
                    self.assignments.insert(float_index, *val);
                }
            }
        }
    }

    pub fn run_decode(self: &mut Self, program : &Program) {
        for expression in &program.expressions {
            self.run_decode_expression(&expression)
        }
    }

    pub fn sum_variables(self : &Self) -> u64 {
        self.assignments.values().sum()
    }
}