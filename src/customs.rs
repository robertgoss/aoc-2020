use std::collections::HashMap;

pub struct Form {
    people : usize,
    answers : HashMap<char, usize>
}
impl Form {
    pub fn new(string : &str) -> Form {
        let chars = string.chars().filter(
            |ch| ch.is_ascii_alphabetic()
        );
        let mut answers = HashMap::new();
        for ch in chars {
            let counter = answers.entry(ch).or_insert(0);
            *counter += 1;
        }
        let people = string.lines().count();
        Form { people : people, answers : answers }
    }
    pub fn num_any(self : &Self) -> usize {
        self.answers.values().filter(
            |&&val| val > 0
        ).count()
    }
    pub fn num_all(self : &Self) -> usize {
        self.answers.values().filter(
            |&&val| val == self.people
        ).count()
    }
}