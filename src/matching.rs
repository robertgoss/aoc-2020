use std::collections::HashMap;

pub enum Rule {
    Lit(String),
    Extern(Vec<usize>),
    Pipe(Box<Rule>, Box<Rule>),
    AStar(usize),
    Bracket(usize, usize)
}

pub struct RuleSet {
    rules : HashMap<usize, Rule>
}

impl Rule {

    fn try_char_from_string(string : &str) -> Option<Rule> {
        if string.len() == 3 && string.contains("\"") {
            Some(Rule::Lit(string[1..2].to_string()))
        } else {
            None
        }
    }

    fn try_pipe_from_string(string : &str) -> Option<Rule> {
        string.split_once(" | ").map(
            |(left, right)| 
                Rule::Pipe(
                    Box::new(Rule::from_string(left)), 
                    Box::new(Rule::from_string(right))
                )
        )
    }

    fn extern_from_string(string : &str) -> Rule {
        let indices = string.split(" ").filter_map(
            |part| part.parse::<usize>().ok()
        ).collect();
        Rule::Extern(indices)
    }

    fn from_string(string : &str) -> Rule {
        Rule::try_pipe_from_string(string).or(
            Rule::try_char_from_string(string)
        ).unwrap_or(
            Rule::extern_from_string(string)
        )
    }

    pub fn from_line(string : &str) -> Option<(usize, Rule)> {
        string.split_once(": ").and_then(
            |(num_str, rule_str)| num_str.parse::<usize>().ok().map(
                |num| (num, Rule::from_string(rule_str))
            )
        )
    }
}

impl RuleSet {
    pub fn from_lines<I>(lines : I) -> RuleSet
      where I : Iterator<Item = String>
    {
        let rules = lines.filter_map(
            |line| Rule::from_line(&line)
        ).collect();
        RuleSet { rules : rules } 
    }

    fn sub_matches_rule<'a>(self : &Self, string : &'a str, rule : &Rule) -> Vec<&'a str> {
        match rule {
            Rule::Lit(lit) => string.strip_prefix(lit).into_iter().collect(),
            Rule::Extern(sub_rules) => {
                let mut remainders : Vec<&'a str> = vec!(string);
                for sub_rule in sub_rules {
                    remainders = remainders.iter().map(
                        |rem_str| self.sub_matches(rem_str, *sub_rule)
                    ).flatten().collect();
                }
                remainders
            }
            Rule::Pipe(left, right) => {
                let mut matches = self.sub_matches_rule(string, left);
                matches.extend(
                    self.sub_matches_rule(string, right).iter()
                );
                matches
            }
            Rule::AStar(rule) => {
                let mut matches = Vec::new();
                let mut new_matches = vec!(string);
                while !new_matches.is_empty() {
                    let base_match = new_matches.pop().unwrap();
                    for new_match in self.sub_matches(base_match, *rule) {
                        new_matches.push(new_match);
                        matches.push(new_match);
                    }
                }
                matches
            }
            Rule::Bracket(open, close) => {
                let mut matches = Vec::new();
                // Wrie AStar track number of opens
                let mut all_open_matches : HashMap<usize, Vec<&'a str>> = HashMap::new();
                let mut new_matches = vec!(string);
                let mut open_depth : usize = 1;
                while !new_matches.is_empty() {
                    let mut open_matches = Vec::new();
                    for base_match in new_matches.iter() {
                        for new_match in self.sub_matches(base_match, *open) {
                            open_matches.push(new_match);
                        }
                    }
                    new_matches = open_matches.clone();
                    all_open_matches.insert(open_depth, open_matches);
                    open_depth += 1;
                }
                for close_depth in 1..open_depth {
                    for open_match in all_open_matches[&close_depth].iter() {
                        for full_match in self.repeat_sub_matches(open_match, *close, close_depth) {
                            matches.push(full_match);
                        }
                    }
                }
                matches
            }
        }
    }

    fn repeat_sub_matches<'a>(self : &Self, string : &'a str, index : usize, repeat : usize) -> Vec<&'a str> {
        let mut matches : Vec<&'a str> = vec!(string);
        for _ in 0..repeat {
            matches = matches.iter().map(
                |matching| self.sub_matches(matching, index)
            ).flatten().collect();
        }
        matches
    }

    fn sub_matches<'a>(self : &Self, string : &'a str, index : usize) -> Vec<&'a str> {
        self.sub_matches_rule(string, self.rules.get(&index).unwrap())
    }

    pub fn is_match(self : &Self, string : &str) -> bool {
        self.sub_matches(string, 0).iter().filter(
            |rest| rest.len() == 0
        ).count() > 0
    }

    pub fn add_new_rules(self : &mut Self){
        self.rules.insert(8, Rule::AStar(42));
        self.rules.insert(11, Rule::Bracket(42,31));
    }

    
}