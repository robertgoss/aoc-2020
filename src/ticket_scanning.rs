use std::collections::HashMap;

pub struct Rule {
    pub name : String,
    ranges : Vec<(u64,u64)>
}

fn parse_range(string : &str) -> Option<(u64, u64)> {
    string.split_once("-").and_then(
        |(min_str, max_str)| min_str.parse::<u64>().ok().and_then(
            |min| max_str.parse::<u64>().ok().map(
                |max| (min, max)
            )
        )
    )
}

impl Rule {
    pub fn from_string(string : &str) -> Option<Rule> {
        string.split_once(": ").map(
            |(name, ranges_str)| {
                Rule {
                    name : name.to_string(),
                    ranges : ranges_str.split(" or ").filter_map(parse_range).collect()
                }
            }
        )
    }

    pub fn valid(self : &Self, val : u64) -> bool {
        self.ranges.iter().any(
            |(min, max)| (*min <= val) && (val <= *max)
        )
    }
}

pub struct Ticket {
    fields : Vec<u64>
}

impl Ticket {
    pub fn from_string(string : &str) -> Ticket {
        Ticket {
            fields : string.split(',').filter_map(
                |part| part.parse::<u64>().ok()
            ).collect()
        }
    }

    pub fn num_invalid_fields(self : &Self, rules : &Vec<Rule>) -> u64 {
        self.fields.iter().filter(
            |val| rules.iter().all(
                |rule| !rule.valid(**val)
            )
        ).sum()
    }

    pub fn field(self : &Self, index : usize) -> u64 {
        self.fields[index]
    }

    pub fn num_fields(self : &Self) -> usize {
        self.fields.len()
    }
}

pub struct ScanningResult {
    rules : Vec<Rule>,
    own_ticket : Ticket,
    other_tickets : Vec<Ticket>
}

impl ScanningResult {
    pub fn from_lines<'a, I, J>(rules_lines : I, own_ticket_str : &str, other_tickets_lines : J) -> ScanningResult 
      where I : Iterator<Item = &'a str>, 
            J : Iterator<Item = &'a str>  
    {
        ScanningResult {
            rules : rules_lines.filter_map(Rule::from_string).collect(),
            own_ticket : Ticket::from_string(own_ticket_str),
            other_tickets : other_tickets_lines.map(Ticket::from_string).collect()
        }
    }

    pub fn scanning_error_rate(self : &Self) -> u64 {
        self.other_tickets.iter().map(
            |ticket| ticket.num_invalid_fields(&self.rules)
        ).sum()
    }

    pub fn discard_invalid(self : &mut Self) {
        let rules = &self.rules;
        let other_tickets = &mut self.other_tickets;
        other_tickets.retain(
            |ticket| ticket.num_invalid_fields(rules) == 0
        );
    }

    fn index_valid_for_rule(self : &Self, rule : &Rule, index : usize) -> bool {
        self.other_tickets.iter().all(
            |ticket| rule.valid(ticket.field(index))
        )
    }

    fn possible_field_indices_for_rule(self : &Self, rule : &Rule) -> Vec<usize> {
        let num_fields = self.own_ticket.num_fields();
        (0..num_fields).filter(
            |index| self.index_valid_for_rule(rule, *index)
        ).collect()
    }

    fn field_indices(self : &Self) -> HashMap<String, usize> {
        let mut field_indices : HashMap<String, usize> = HashMap::new();
        let mut possible_indices : HashMap<String, Vec<usize>> = 
          self.rules.iter().map(
              |rule| (rule.name.to_string(), self.possible_field_indices_for_rule(rule))
          ).collect();
        while !possible_indices.is_empty() {
            let (rule, index) = possible_indices.iter().filter(
                |(_, indices)| indices.len() == 1
            ).map(
                |(name, indices)| (name.to_string(), indices[0])
            ).next().unwrap();
            possible_indices.remove(&rule);
            for val in possible_indices.values_mut() {
                val.retain(|i| *i != index)
            }
            field_indices.insert(rule, index);
        }
        field_indices
    }

    pub fn departures(self : &Self) -> Vec<String> {
        self.rules.iter().filter(
            |rule| rule.name.starts_with("departure")
        ).map(
            |rule| rule.name.to_string()
        ).collect()
    }

    pub fn own_field(self : &Self, name : &str) -> Option<u64> {
        self.field_indices().get(name).map(
            |index| self.own_ticket.field(*index)
        )
    }
}