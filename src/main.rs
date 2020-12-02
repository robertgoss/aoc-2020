#![feature(str_split_once)]
mod expenses {
    use std::collections::HashSet;
    use std::iter::FromIterator;


    // find pair of numbers in values list that sum together to make total
    fn _find_2_summands(value_set : &HashSet<i64>, total : i64) -> Option<(i64, i64)> {
        value_set.iter().filter(
            |&value| value_set.contains(&(total - value))
        ).next().map(
            |&summand| (summand, total - summand)
        )
    }

    // find triple of numbers in values list that sum together to make total
    fn _find_3_summands(value_set : &HashSet<i64>, total : i64) -> Option<(i64, i64, i64)> {
        value_set.iter().filter_map(
            |&value| _find_2_summands(value_set, total - value)
        ).next().map(
            |(a, b)| (a, b, total - a - b)
        )
    }

    // find pair of numbers in values list that sum together to make total
    pub fn find_2_summands(values : &Vec<i64>, total : i64) -> Option<(i64, i64)> {
        // Query set to avoid quadratic check
        let value_set : HashSet<i64> = 
          HashSet::from_iter(values.iter().cloned());
        _find_2_summands(&value_set, total)
    }

    // find triple of numbers in values list that sum together to make total
    pub fn find_3_summands(values : &Vec<i64>, total : i64) -> Option<(i64, i64, i64)> {
        // Query set to avoid quadratic check
        let value_set : HashSet<i64> = 
          HashSet::from_iter(values.iter().cloned());
        _find_3_summands(&value_set, total)
    }
}

mod passwords {
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

}

mod io {
    use std::io::BufRead;
    use std::fs::File;
    use std::io::BufReader;
    use super::passwords as passwords;

    pub fn input_as_list(day: i8) -> Vec<i64> {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        reader.lines().map(
            |s| s.expect("Read failure").parse::<i64>().unwrap()
        ).collect()
    }

    pub fn input_as_password_database(day: i8) -> passwords::Database {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        let mut database = passwords::Database::new();
        for line in reader.lines() {
            database.add_line(&line.expect("Read failure"));
        }
        database
    }
}

mod challenge {
    use super::io as io;
    use super::expenses as expenses;
    use super::passwords as passwords;

    fn challenge_1() {
        let data = io::input_as_list(1);
        let (a, b) = expenses::find_2_summands(&data, 2020).unwrap();
        println!("{} {} {}", a , b, a * b);
    }

    fn challenge_2() {
        let data = io::input_as_list(1);
        let (a, b, c) = expenses::find_3_summands(&data, 2020).unwrap();
        println!("{} {} {} {}", a, b, c, a * b * c);
    }

    fn challenge_3() {
        let data = io::input_as_password_database(2);
        let num = data.count_valid(passwords::RuleSet::Sled);
        println!("{}", num);
    }
    fn challenge_4() {
        let data = io::input_as_password_database(2);
        let num = data.count_valid(passwords::RuleSet::Toboggan);
        println!("{}", num);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(4);
}