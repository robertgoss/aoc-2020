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