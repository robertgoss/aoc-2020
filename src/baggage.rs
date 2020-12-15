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