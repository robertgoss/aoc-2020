#![feature(str_split_once)]
#![feature(iter_map_while)]
#![feature(unsigned_abs)]
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

mod baggage {
    use std::collections::HashMap;
    use petgraph::prelude::Dfs;
    use petgraph::Graph;
    use petgraph::graph::NodeIndex;
    use petgraph::visit::Reversed;
    use petgraph::visit::EdgeRef;
    use petgraph::Outgoing;

    fn parse_bag(bag : &str) -> Option<(String, usize)> {
        let parts : Vec<&str> = bag.split(" ").collect();
        if parts.len() < 3 {
            return None
        }
        if parts[0].parse::<usize>().is_err() {
            return None
        }
        let mut bag_name : String = parts[1].to_string();
        bag_name.push_str(" ");
        bag_name.push_str(parts[2]);
        return Some( (bag_name, parts[0].parse::<usize>().unwrap()) )
    }

    fn parse_contents(contents : &str) -> Vec<(String, usize)> {
        if contents == "no other bags." {
            return Vec::new()
        }
        contents.split(", ").filter_map(
            |bag| parse_bag(bag)
        ).collect()
    }

    pub struct Rules {
        colours : HashMap<String, NodeIndex<u32>>,
        rules : Graph::<String, usize>
    }

    impl Rules {
        pub fn new() -> Rules {
            Rules { 
                colours : HashMap::<String, NodeIndex<u32>>::new(),
                rules : Graph::<String, usize>::new() 
            }
        }

        pub fn add_line(self : &mut Self, line : &str) {
            let parsed = line.split_once(" bags contain ").map(
                |(bag, contents)| (bag, parse_contents(contents))
            );
            match parsed {
                Some((bag, contents)) => self.add_rule(bag, contents),
                _ => ()
            }
        }

        fn add_rule(self : &mut Self, bag : &str, contents : Vec<(String, usize)>) {
            let bag_id = self.add_colour(bag);
            for (contained_bag, num) in contents.iter() {
                let contained_bag_id  = self.add_colour(contained_bag);
                self.rules.add_edge(bag_id, contained_bag_id, *num);
            }
        }

        fn add_colour(self : &mut Self, bag : &str) -> NodeIndex<u32> {
            match self.colours.get(&bag.to_string()) {
                Some(bag_id) => *bag_id,
                None => {
                    let id = self.rules.add_node(bag.to_string());
                    self.colours.insert(bag.to_string(), id);
                    id
                }
            }
        }

        pub fn num_dependencies_node(self : &Self, node : NodeIndex<u32>) -> usize {
            // Iterate backwards through rules
            let reverse_rules = Reversed(&self.rules);
            let mut dfs = Dfs::new(&reverse_rules, node);
            let mut count : usize = 0;
            while let Some(nx) = dfs.next(&reverse_rules) {
                if nx!= node {
                    count += 1;
                }
            }
            count
        }

        pub fn num_dependencies(self : &Self, node : &str) -> Option<usize> {
            self.colours.get(&node.to_string()).map(
                |start_id| self.num_dependencies_node(*start_id)
            )
        }

        pub fn full_num_contained_node(self : &Self, node : NodeIndex<u32>) -> usize {
            self.rules.edges_directed(node, Outgoing).map(
                |edge| edge.weight() * (1 + self.full_num_contained_node(edge.target()))
            ).sum()
        }

        pub fn full_num_contained(self : &Self, node : &str) -> Option<usize> {
            self.colours.get(&node.to_string()).map(
                |start_id| self.full_num_contained_node(*start_id)
            )
        }
    }
}

mod cpu {
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
}

mod cipher {

    pub struct CipherText {
        data : Vec<usize>
    }

    fn prop_holds(window : &[usize]) -> bool {
        let last = *window.last().unwrap();
        for i in 0..(window.len()-1) {
            for j in i..(window.len()-1) {
                if last == window[i] + window[j] {
                    return true;
                }
            }
        }
        return false;
    }

    impl CipherText {
        pub fn from_lines<I>(lines : I) -> CipherText 
          where I : Iterator<Item = String> 
        {
            CipherText {
                data : lines.filter_map(
                    |line| line.parse::<usize>().ok()
                ).collect()
            }
        }

        pub fn first_prop(self : &Self, block : usize) -> Option<usize> {
            self.data.windows(block+1).filter(
                |slice| !prop_holds(slice)
            ).map(
                |slice| slice[block]
            ).next()
        }

        pub fn weakness(self : &Self, block : usize) -> Option<usize> {
            let val = self.first_prop(block).unwrap();
            let len = self.data.len();
            for i in 0..(len-1) {
                for j in (i+1)..len {
                    if val == self.data[i..=j].iter().sum::<usize>() {
                        let smallest = self.data[i..=j].iter().min().unwrap();
                        let largest = self.data[i..=j].iter().max().unwrap();
                        return Some(smallest + largest)
                    }
                }
            }
            return None;
        }
    }
}

mod adaptors {
    use std::collections::HashMap;

    pub struct Adaptors {
        full_chain : Vec<usize>
    }

    impl Adaptors {
        pub fn from_lines<I>(lines : I) -> Adaptors 
          where I : Iterator<Item = String> 
        {
            let mut full_chain : Vec<usize> = lines.filter_map(
                |line| line.parse::<usize>().ok()
            ).collect();
            full_chain.push(0);
            full_chain.push(*full_chain.iter().max().unwrap()+3);
            full_chain.sort_unstable();
            Adaptors {
                full_chain : full_chain
            }
        }

        pub fn joltage_differences(self : &Self) -> usize {
            let one_differences = self.full_chain.windows(2).filter(
                |slice| slice[1] == slice[0]+1
            ).count();
            let three_differences = self.full_chain.windows(2).filter(
                |slice| slice[1] == slice[0]+3
            ).count();
            one_differences * three_differences
        }

        fn number_arrangements_start_cached(
            self : &Self, 
            start_index : usize, 
            cache : &mut HashMap<usize, usize>
        ) -> usize {
            let chain_len = self.full_chain.len();
            // Base cases
            if start_index == chain_len - 1 {
                return 1;
            }
            if cache.contains_key(&start_index) {
                return cache[&start_index];
            }
            let mut count = 0;
            // Get the number of arangements is we take the next elements (max 3)
            let start_val = self.full_chain[start_index];
            for delta in 1..=3 {
                let next_index = start_index + delta;
                if next_index < chain_len {
                    let next_val = self.full_chain[next_index];
                    if next_val <= start_val + 3 {
                        count += self.number_arrangements_start_cached(next_index, cache);
                    }
                }
            }
            cache.insert(start_index, count);
            count
        }

        pub fn number_arrangements(self : &Self) -> usize {
            let mut cache : HashMap<usize, usize> = HashMap::new();
            self.number_arrangements_start_cached(0, &mut cache)
        }
    }
}

mod seating {
    use std::collections::HashMap;

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum SeatState {
        Floor,
        Occupied,
        Empty
    }

    pub enum SeatingRules {
        Adjacent,
        Visible
    }

    impl SeatState {
        fn from_char(ch : char) -> SeatState {
            match ch {
                'L' => SeatState::Empty,
                '#' => SeatState::Occupied,
                _ => SeatState::Floor
            }
        }
    }


    pub struct Seating {
        seats : HashMap<(i32, i32), SeatState>
    }

    impl Seating {
        pub fn from_lines<I>(lines : I) -> Seating 
          where I : Iterator<Item = String> 
        {
            let mut seats : HashMap<(i32, i32), SeatState> = HashMap::new();
            for (i, line) in lines.enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    seats.insert((i as i32, j as i32) , SeatState::from_char(ch));
                }
            }
            Seating { seats : seats }
        }

        fn is_occupied(self : &Self, seat : (i32, i32)) -> bool {
            self.seats.get(&seat).map(
                |&state| state == SeatState::Occupied
            ).unwrap_or(false)
        }

        fn surrounding_occupied(self : &Self, (seat_i,seat_j) : &(i32, i32)) -> usize {
            let mut count : usize = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i != 0 || j != 0 {
                        if self.is_occupied((seat_i + i, seat_j + j)) {
                            count += 1;
                        }
                    }
                }
            }
            count
        }

        fn is_direction_occupied(self : &Self, (dir_i,dir_j) : (i32, i32), (seat_i,seat_j) : &(i32, i32)) -> bool {
            (1..).map(
                |k| (seat_i + (k*dir_i), seat_j + (k*dir_j))
            ).map_while(
                |seat| self.seats.get(&seat)
            ).filter(
                |&&state| state != SeatState::Floor
            ).next().map(
                |&state| state == SeatState::Occupied
            ).unwrap_or(false)
        }

        fn visible_occupied(self : &Self, seat : &(i32, i32)) -> usize {
            let mut count : usize = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i != 0 || j != 0 {
                        if self.is_direction_occupied((i,j), seat) {
                            count += 1;
                        }
                    }
                }
            }
            count
        }

        fn seat_next_state_adjacent(self : &Self, seat : &(i32, i32), state : &SeatState) -> SeatState {
            match state {
                SeatState::Empty => {
                    if self.surrounding_occupied(seat) == 0 { 
                        SeatState::Occupied 
                    } else { 
                        SeatState::Empty 
                    }
                },
                SeatState::Occupied => {
                    if self.surrounding_occupied(seat) >= 4 {
                        SeatState::Empty
                    } else {
                        SeatState::Occupied
                    }
                },
                SeatState::Floor => SeatState::Floor
            }
        }

        fn seat_next_state_visible(self : &Self, seat : &(i32, i32), state : &SeatState) -> SeatState {
            match state {
                SeatState::Empty => {
                    if self.visible_occupied(seat) == 0 { 
                        SeatState::Occupied 
                    } else { 
                        SeatState::Empty 
                    }
                },
                SeatState::Occupied => {
                    if self.visible_occupied(seat) >= 5 {
                        SeatState::Empty
                    } else {
                        SeatState::Occupied
                    }
                },
                SeatState::Floor => SeatState::Floor
            }
        }

        fn seat_next_state(self : &Self, seat : &(i32, i32), state : &SeatState, rules : &SeatingRules) -> SeatState {
            match rules {
                SeatingRules::Adjacent => self.seat_next_state_adjacent(seat,state),
                SeatingRules::Visible => self.seat_next_state_visible(seat,state)
            }
        }

        pub fn simulate_once(self : &mut Self, rules : &SeatingRules) -> bool {
            let new_seats : HashMap<(i32, i32), SeatState> = 
              self.seats.iter().map(
                |(seat, state)| (*seat, self.seat_next_state(seat, state, rules))
              ).collect();
            let changed : bool = self.seats != new_seats;
            self.seats = new_seats;
            changed
        }

        pub fn simulate(self : &mut Self, rules : &SeatingRules) {
            let mut changed : bool = true;
            while changed {
                changed = self.simulate_once(rules);
            }
        }

        pub fn number_occupied(self : &Self) -> usize {
            self.seats.values().filter(
                |&&val| val == SeatState::Occupied
            ).count()
        }
    }
}

mod directions {
    #[derive(Copy, Clone, Debug)]
    enum Cardinal {
        North,
        East,
        South,
        West
    }
    impl Cardinal {
        fn next(self : &Self) -> Self {
            match self {
                Cardinal::North => Cardinal::East,
                Cardinal::East => Cardinal::South,
                Cardinal::South => Cardinal::West,
                Cardinal::West => Cardinal::North
            }
        } 
        fn prev(self : &Self) -> Self {
            match self {
                Cardinal::North => Cardinal::West,
                Cardinal::East => Cardinal::North,
                Cardinal::South => Cardinal::East,
                Cardinal::West => Cardinal::South
            }
        } 
        pub fn rotate(self : &Self, amount : i32) -> Self {
            if amount == 0 {
                *self
            } else if amount > 0 {
                self.next().rotate(amount-1)
            } else {
                self.prev().rotate(amount+1)
            }
        }
    }
    #[derive(Copy, Clone)]
    enum ActionType {
        Direction(Cardinal),
        Left,
        Right,
        Forward
    }
    impl ActionType {
        pub fn from_char(ch : char) -> Option<ActionType> {
            match ch {
                'L' => Some(ActionType::Left),
                'R' => Some(ActionType::Right),
                'F' => Some(ActionType::Forward),
                'N' => Some(ActionType::Direction(Cardinal::North)),
                'S' => Some(ActionType::Direction(Cardinal::South)),
                'E' => Some(ActionType::Direction(Cardinal::East)),
                'W' => Some(ActionType::Direction(Cardinal::West)),
                _ => None
            }
        }
    }
    pub struct Action {
        atype : ActionType,
        val : i32
    }
    impl Action {
        pub fn from_string(string : &str) -> Option<Action> {
            ActionType::from_char(
                string.chars().next().unwrap()
            ).and_then(
                |atype| string[1..].parse::<i32>().ok().map(
                    |val| Action{ atype : atype, val : val}
                )
            )
        }
    }
    pub struct Ship {
        position : (i32, i32),
        waypoint : (i32, i32),
        direction : Cardinal
    }
    impl Ship {
        pub fn new() -> Ship {
            Ship { position : (0,0), direction : Cardinal::East, waypoint : (10, 1) }
        }

        pub fn simulate<I>(self : &mut Self, actions : I)
          where I : Iterator<Item = Action>
        {
            for action in actions {
                self.simulate_once(&action);
            }
        }

        pub fn simulate_waypoint<I>(self : &mut Self, actions : I)
          where I : Iterator<Item = Action>
        {
            for action in actions {
                self.simulate_waypoint_once(&action);
            }
        }

        fn simulate_once(self : &mut Self, action : &Action)
        {
            match action.atype {
                ActionType::Direction(dir) => self.move_direction(dir, action.val),
                ActionType::Forward => self.move_direction(self.direction, action.val),
                ActionType::Left => self.turn(-action.val),
                ActionType::Right => self.turn(action.val)
            } 
        }

        fn simulate_waypoint_once(self : &mut Self, action : &Action)
        {
            match action.atype {
                ActionType::Direction(dir) => self.move_waypoint(dir, action.val),
                ActionType::Forward => self.move_to_waypoint(action.val),
                ActionType::Left => self.turn_waypoint(-action.val / 90),
                ActionType::Right => self.turn_waypoint(action.val / 90)
            } 
        }

        fn turn(self : &mut Self, degrees : i32) {
            let amount : i32 = degrees / 90;
            self.direction = self.direction.rotate(amount);
        }

        fn move_direction(self : &mut Self, direction : Cardinal, distance : i32) {
            match direction {
                Cardinal::North => self.position.1 += distance,
                Cardinal::South => self.position.1 -= distance,
                Cardinal::East => self.position.0 += distance,
                Cardinal::West => self.position.0 -= distance
            };
        }

        fn move_waypoint(self : &mut Self, direction : Cardinal, distance : i32) {
            match direction {
                Cardinal::North => self.waypoint.1 += distance,
                Cardinal::South => self.waypoint.1 -= distance,
                Cardinal::East => self.waypoint.0 += distance,
                Cardinal::West => self.waypoint.0 -= distance
            };
        }

        fn move_to_waypoint(self : &mut Self, distance : i32) {
            self.position.0 = self.position.0 + (self.waypoint.0 * distance);
            self.position.1 = self.position.1 + (self.waypoint.1 * distance);
        }

        fn turn_waypoint_prev(self : &mut Self) {
            self.waypoint = (-self.waypoint.1, self.waypoint.0)
        }

        fn turn_waypoint_next(self : &mut Self) {
            self.waypoint = (self.waypoint.1, -self.waypoint.0)
        }

        fn turn_waypoint(self : &mut Self, amount : i32) {
            if amount > 0 {
                self.turn_waypoint_next();
                self.turn_waypoint(amount-1);
            } else if amount < 0 {
                self.turn_waypoint_prev();
                self.turn_waypoint(amount+1);
            }
        }

        pub fn distance(self : &Self) -> u32 {
            self.position.0.unsigned_abs() + self.position.1.unsigned_abs()
        }
    }

}

mod buses {
    #[derive(Copy, Clone)]
    pub struct Bus {
        period : Option<u64>
    }
    impl Bus {
        pub fn from_string(string : &str) -> Bus {
            Bus {
                period : string.parse::<u64>().ok()
            }
        }

        pub fn id(self : &Self) -> u64 {
            self.period.unwrap()
        }

        pub fn departs(self : &Self, timestamp : u64) -> bool {
            self.period.map(
                |time| timestamp % time == 0
            ).unwrap_or(false)
        }
    }
    pub struct Timetable {
        timestamp : u64,
        buses : Vec<Bus>
    }
    impl Timetable {
        pub fn from_lines(line1 : &str, line2 : &str) -> Timetable {
            Timetable {
                timestamp : line1.parse::<u64>().unwrap(),
                buses : line2.split(",").map(
                    |string| Bus::from_string(string)
                ).collect()
            }
        }

        pub fn first_bus(self : &Self) -> Option<(u64, Bus)> {
            (0..).filter_map(
                |index| self.has_bus(index + self.timestamp).map(
                    |bus| (index, bus)
                )
            ).next()
        }

        fn has_bus(self : &Self, timestamp : u64) -> Option<Bus> {
            self.buses.iter().filter(
                |bus| bus.departs(timestamp)
            ).next().map(
                |bus| *bus
            )
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
    use super::baggage as baggage;
    use super::cpu as cpu;
    use super::cipher as cipher;
    use super::adaptors as adaptors;
    use super::seating as seating;
    use super::directions as directions;
    use super::buses as buses;

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

    pub fn input_as_rules(day : i8) -> baggage::Rules {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        let mut rules = baggage::Rules::new();
        for line in reader.lines() {
            rules.add_line(&line.expect("Read failure"));
        }
        rules
    }

    pub fn input_as_program(day : i8) -> cpu::Program {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        cpu::Program::from_lines(
            reader.lines().map(|line| line.expect("Read failure"))
        )
    }

    pub fn input_as_ciphertext(day : i8) -> cipher::CipherText {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        cipher::CipherText::from_lines(
            reader.lines().map(|line| line.expect("Read failure"))
        )
    }

    pub fn input_as_adaptors(day : i8) -> adaptors::Adaptors {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        adaptors::Adaptors::from_lines(
            reader.lines().map(|line| line.expect("Read failure"))
        )
    }

    pub fn input_as_seating(day : i8) -> seating::Seating {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        seating::Seating::from_lines(
            reader.lines().map(|line| line.expect("Read failure"))
        )
    }

    pub fn input_as_actions(day : i8) -> Vec<directions::Action> {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        reader.lines().filter_map(
            |line| directions::Action::from_string(&line.expect("Read failure"))
        ).collect()
    }

    pub fn input_as_timetable(day : i8) -> buses::Timetable {
        let filename = format!("data/day-{}.txt", day);
        let file = File::open(filename).expect("Issue opening file");
        let reader = BufReader::new(&file);
        let lines : Vec<String> = reader.lines().map(
            |line| line.expect("Read failure")
        ).collect();
        buses::Timetable::from_lines(
            &lines[0],
            &lines[1]
        )
    }
}

mod challenge {
    use super::io as io;
    use super::expenses as expenses;
    use super::passwords as passwords;
    use super::seating as seating;
    use super::directions as directions;

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
    fn challenge_13() {
        let data = io::input_as_rules(7);
        let num = data.num_dependencies("shiny gold").unwrap();
        println!("{}", num);
    }
    fn challenge_14() {
        let data = io::input_as_rules(7);
        let num = data.full_num_contained("shiny gold").unwrap();
        println!("{}", num);
    }
    fn challenge_15() {
        let data = io::input_as_program(8);
        let num = data.run();
        println!("{}", num);
    }
    fn challenge_16() {
        let data = io::input_as_program(8);
        let num = data.fix().unwrap();
        println!("{}", num);
    }
    fn challenge_17() {
        let data = io::input_as_ciphertext(9);
        let num = data.first_prop(25).unwrap();
        println!("{}", num);
    }
    fn challenge_18() {
        let data = io::input_as_ciphertext(9);
        let num = data.weakness(25).unwrap();
        println!("{}", num);
    }
    fn challenge_19() {
        let data = io::input_as_adaptors(10);
        let num = data.joltage_differences();
        println!("{}", num);
    }
    fn challenge_20() {
        let data = io::input_as_adaptors(10);
        let num = data.number_arrangements();
        println!("{}", num);
    }
    fn challenge_21() {
        let mut data = io::input_as_seating(11);
        data.simulate(&seating::SeatingRules::Adjacent);
        let num = data.number_occupied();
        println!("{}", num);
    }
    fn challenge_22() {
        let mut data = io::input_as_seating(11);
        data.simulate(&seating::SeatingRules::Visible);
        let num = data.number_occupied();
        println!("{}", num);
    }
    fn challenge_23() {
        let data = io::input_as_actions(12);
        let mut ship = directions::Ship::new();
        ship.simulate(data.into_iter());
        let num = ship.distance();
        println!("{}", num);
    }
    fn challenge_24() {
        let data = io::input_as_actions(12);
        let mut ship = directions::Ship::new();
        ship.simulate_waypoint(data.into_iter());
        let num = ship.distance();
        println!("{}", num);
    }
    fn challenge_25() {
        let data = io::input_as_timetable(13);
        let (offset, bus) = data.first_bus().unwrap();
        let num = offset * bus.id();
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
            13 => challenge_13(),
            14 => challenge_14(),
            15 => challenge_15(),
            16 => challenge_16(),
            17 => challenge_17(),
            18 => challenge_18(),
            19 => challenge_19(),
            20 => challenge_20(),
            21 => challenge_21(),
            22 => challenge_22(),
            23 => challenge_23(),
            24 => challenge_24(),
            25 => challenge_25(),
            _ => () 
        }
    }
}



fn main() {
    challenge::challenge(25);
}