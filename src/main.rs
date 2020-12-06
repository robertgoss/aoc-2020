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

mod map {
    use std::cmp;
    use std::collections::HashSet;

    pub struct Map {
        length : usize,
        period : usize,
        trees : HashSet<(usize, usize)>
    }
    impl Map {
        pub fn new() -> Map {
            Map {length : 0, period : 0, trees : HashSet::new()}
        }
        fn lookup_tree(self : &Self, i : usize, j : usize) -> bool {
            // Reduce j to the area to look at
            let reduced_j : usize = j % self.period;
            self.trees.contains(&(i, reduced_j))
        }
        pub fn count_trees_path(self : &Self, i_step : usize, j_step : usize) -> usize {
            let step_num = 1 + (self.length / i_step);
            (0..step_num).filter(
                |&i| self.lookup_tree(i_step * i, j_step * i)
            ).count()
        }
        pub fn add_line(self : &mut Self, line : &str) {
            for (index, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.trees.insert((self.length, index));
                }
            }
            self.length += 1;
            self.period = cmp::max(self.period, line.chars().count())
        }
    }
}

mod passport {
    use std::iter::FromIterator;
    use std::collections::HashMap;

    static REQUIRED_FIELD_NAMES : [&str; 7] = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    static VALID_EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    fn is_hex(string : &str) -> bool {
        string.chars().all(
            |c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c)
        )
    }

    pub struct Passport {
        elements : HashMap<String, String>
    }
    impl Passport {
        pub fn new(data : &str) -> Passport {
            let data = data.to_string();
            let elements = HashMap::from_iter(
                data.split_whitespace().filter_map(
                    |part| part.split_once(':')
                ).map(
                    |(key, val)| (key.to_string(), val.to_string())
                )
            );
            Passport { elements : elements }
        }
        pub fn required_fields_exist(self : &Self) -> bool {
            REQUIRED_FIELD_NAMES.iter().all(
                |field_name| self.elements.contains_key(&field_name.to_string())
            )
        }
        pub fn required_fields_valid(self : &Self) -> bool {
            self.byr_valid() &&
            self.iyr_valid() &&
            self.eyr_valid() &&
            self.hgt_valid() &&
            self.hcl_valid() &&
            self.ecl_valid() &&
            self.pid_valid()
        }

        pub fn byr_valid(self : &Self) -> bool {
            self.elements.get("byr").and_then(
                |byr| byr.parse::<usize>().ok()
            ).map(
                |byr| 1920 <= byr && byr <= 2002
            ).unwrap_or(false)
        }

        fn iyr_valid(self : &Self) -> bool {
            self.elements.get("iyr").and_then(
                |byr| byr.parse::<usize>().ok()
            ).map(
                |byr| 2010 <= byr && byr <= 2020
            ).unwrap_or(false)
        }

        fn eyr_valid(self : &Self) -> bool {
            self.elements.get("eyr").and_then(
                |byr| byr.parse::<usize>().ok()
            ).map(
                |byr| 2020 <= byr && byr <= 2030
            ).unwrap_or(false)
        }

        fn hgt_valid(self : &Self) -> bool {
            self.hgt_valid_cm() || self.hgt_valid_in()
        }

        fn hgt_valid_cm(self : &Self) -> bool {
            self.elements.get("hgt").and_then(
                |hgt| hgt.strip_suffix("cm")
            ).and_then(
                |hgt| hgt.parse::<usize>().ok()
            ).map(
                |hgt| (150..=193).contains(&hgt)
            ).unwrap_or(false)
        }

        fn hgt_valid_in(self : &Self) -> bool {
            self.elements.get("hgt").and_then(
                |hgt| hgt.strip_suffix("in")
            ).and_then(
                |hgt| hgt.parse::<usize>().ok()
            ).map(
                |hgt| (59..=76).contains(&hgt)
            ).unwrap_or(false)
        }

        fn hcl_valid(self : &Self) -> bool {
            self.elements.get("hcl").and_then(
                |hcl| hcl.strip_prefix('#')
            ).map(
                |hex| is_hex(hex)
            ).unwrap_or(false)
        }

        fn ecl_valid(self : &Self) -> bool {
            self.elements.get("ecl").map(
                |ecl| VALID_EYE_COLOURS.iter().filter(|&col| col == ecl).count() == 1
            ).unwrap_or(false)
        }

        fn pid_valid(self : &Self) -> bool {
            self.elements.get("pid").map(
                |pid| pid.parse::<usize>().is_ok() && pid.len() == 9
            ).unwrap_or(false)
        }
    }
}

mod ticket {
    use std::collections::HashSet;

    fn from_names_binary(zero : char, string : &str) -> Option<usize> {
        let converted : String = string.chars().map(
            |c| if c==zero {'0'} else {'1'}
        ).collect();
        usize::from_str_radix(&converted, 2).ok()
    } 
    pub struct Seat {
        row : usize,
        col : usize
    }

    impl Seat {
        pub fn from_string(string : &str) -> Option<Seat> {
            let row_opt = string.get(..7).and_then(
                |row| from_names_binary('F', row)
            );
            let col_opt = string.get(7..).and_then(
                |col| from_names_binary('L', col)
            );
            match (row_opt, col_opt) {
                (Some(row), Some(col)) => Some(Seat {row: row, col : col}),
                _ => None
            } 
        }
        pub fn id(self : &Self) -> usize {
            self.row * 8 + self.col
        }
    }

    pub struct Plane {
        seats : HashSet<usize>
    }
    impl Plane {
        pub fn new() -> Plane {
            Plane { seats : HashSet::new() }
        }
        pub fn add_seat(self : &mut Self, seat : &Seat) {
            self.seats.insert(seat.id());
        }
        pub fn max(self : &Self) -> usize {
            *self.seats.iter().max().unwrap_or(&0)
        }
        pub fn min(self : &Self) -> usize {
            *self.seats.iter().min().unwrap_or(&0)
        }
        pub fn find_missing(self : &Self) -> Vec<usize> {
            let min = self.min();
            let max = self.max();
            (min..max).filter(
                |num| !self.seats.contains(num)
            ).collect()
        }
    }
    
}

mod customs {
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
}

mod io {
    use std::io::BufRead;
    use std::fs;
    use std::fs::File;
    use std::io::BufReader;
    use super::passwords as passwords;
    use super::map as map;
    use super::passport as passport;
    use super::ticket as ticket;
    use super::customs as customs;

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

    pub fn input_as_map(day: i8) -> map::Map {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        let mut map = map::Map::new();
        for line in reader.lines() {
            map.add_line(&line.expect("Read failure"));
        }
        map
    }

    pub fn input_as_passports(day : i8) -> Vec<passport::Passport> {
        let filename = format!("data/day-{}.txt", day);
        let data = fs::read_to_string(filename).expect("Issue reading file");
        data.split("\n\n").map(
            |chunk| passport::Passport::new(chunk)
        ).collect()
    }

    pub fn input_as_plane(day : i8) -> ticket::Plane {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        let seats : Vec<ticket::Seat> = reader.lines().map(
            |line| ticket::Seat::from_string(&line.expect("ReadFailure")).unwrap()
        ).collect();
        let mut plane = ticket::Plane::new();
        for seat in seats.iter() {
            plane.add_seat(seat);
        }
        plane
    }

    pub fn input_as_forms(day : i8) -> Vec<customs::Form> {
        let filename = format!("data/day-{}.txt", day);
        let data = fs::read_to_string(filename).expect("Issue reading file");
        data.split("\n\n").map(
            |chunk| customs::Form::new(chunk)
        ).collect()
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
    fn challenge_5() {
        let data = io::input_as_map(3);
        let num = data.count_trees_path(1, 3);
        println!("{}", num);
    }
    fn challenge_6() {
        let data = io::input_as_map(3);
        let a = data.count_trees_path(1, 1);
        let b = data.count_trees_path(1, 3);
        let c = data.count_trees_path(1, 5);
        let d = data.count_trees_path(1, 7);
        let e = data.count_trees_path(2, 1);
        println!("{} {} {} {} {} {}", a, b, c, d, e, a*b*c*d*e);
    }
    fn challenge_7() {
        let data = io::input_as_passports(4);
        let num = data.iter().filter(
            |passport| passport.required_fields_exist()
        ).count();
        println!("{}", num);
    }
    fn challenge_8() {
        let data = io::input_as_passports(4);
        let num = data.iter().filter(
            |passport| passport.required_fields_valid()
        ).count();
        println!("{}", num);
    }
    fn challenge_9() {
        let data = io::input_as_plane(5);
        let num = data.max();
        println!("{}", num);
    }
    fn challenge_10() {
        let data = io::input_as_plane(5);
        let num = data.find_missing()[0];
        println!("{}", num);
    }
    fn challenge_11() {
        let data = io::input_as_forms(6);
        let num : usize = data.iter().map(
            |form| form.num_any()
        ).sum();
        println!("{}", num);
    }
    fn challenge_12() {
        let data = io::input_as_forms(6);
        let num : usize = data.iter().map(
            |form| form.num_all()
        ).sum();
        println!("{}", num);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            7 => challenge_7(),
            8 => challenge_8(),
            9 => challenge_9(),
            10 => challenge_10(),
            11 => challenge_11(),
            12 => challenge_12(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(12);
}