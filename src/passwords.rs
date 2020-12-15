#[derive(Copy, Clone)]
pub enum RuleSet {
    Sled,
    Toboggan
}
struct Verification {
    letter : char,
    max : usize,
    min : usize
}
fn parse_range(string : &str) -> Option<(usize, usize)> {
    let parts = string.split_once('-').map(
        |(min, max)| (min.parse::<usize>(), max.parse::<usize>())
    );
    match parts {
        Some((Ok(min), Ok(max))) => Some((min, max)),
        _ => None
    }
}
impl Verification {
    pub fn from_string(string : &str) -> Option<Verification> {
        let parts = string.split_once(' ').map(
            |(range, ch)| (parse_range(range), ch.chars().next())
        );
        match parts {
            Some((Some((min, max)), Some(ch))) => Some(
                Verification {
                    letter : ch,
                    max : max,
                    min : min
                }
            ),
            _ => None
        }
    }
    pub fn verify(self : &Self, string : &str, rules : RuleSet) -> bool {
        match rules {
            RuleSet::Sled => self.verify_sled(string),
            RuleSet::Toboggan => self.verify_toboggan(string)
        }
    }
    pub fn verify_sled(self : &Self, string : &str) -> bool {
        let char_count = string.chars().filter(
            |&ch| ch == self.letter
        ).count();
        char_count >= self.min && char_count <= self.max
    }
    pub fn verify_toboggan(self : &Self, string : &str) -> bool {
        // Seeif the elements on the bounds are as req handle out of bounds
        let min_has = string.chars().nth(self.min - 1).map(
            |ch| ch == self.letter 
        ).unwrap_or(false);
        let max_has = string.chars().nth(self.max - 1).map(
            |ch| ch == self.letter 
        ).unwrap_or(false);
        // Either min xor max should have
        min_has ^ max_has
    }
}
pub struct Database {
    passwords : Vec<(Verification, String)>
}
impl Database {
    pub fn new() -> Database {
        Database { passwords : Vec::new() }
    }
    pub fn add_line(self : &mut Self, line : &str) {
        let parts = line.split_once(':').map(
            |(validation, password)| (
                Verification::from_string(validation), 
                password.strip_prefix(" ")
            )
        );
        // Add if everything valid
        match parts {
            Some((Some(validation), Some(password))) => self.passwords.push((validation, password.to_string())),
            _ => ()
        }
    }
    pub fn count_valid(self : &Self, rules : RuleSet) -> usize {
        self.passwords.iter().filter(
            |(verification, password)| verification.verify(password, rules)
        ).count()
    }
    }