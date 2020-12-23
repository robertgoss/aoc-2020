use itertools::Itertools;

pub struct Cups {
    current_index : usize,
    items : Vec<usize>
}

impl Cups {
    pub fn from_ints(ints : Vec<usize>, total : usize) -> Cups {
        Cups {
            current_index : 0,
            items : ints.iter().cloned().chain(
                ints.len()..total
            ).collect()
        }
    }

    fn take_next(self : &mut Self) -> usize {
        self.current_index = self.current_index % self.items.len();
        let next_index = (self.current_index+1) % self.items.len();
        let item = self.items.remove(next_index);
        if next_index <= self.current_index {
            self.current_index -= 1;
        }
        item
    }

    fn put_after(self : &mut Self, index : usize, val : usize) {
        self.current_index = self.current_index % self.items.len();
        self.items.insert(index + 1, val);
        if index <= self.current_index {
            self.current_index += 1;
        }
    }

    fn destination(self : &Self) -> usize {
        let val = self.get(self.current_index);
        let final_index = self.get_index(*self.items.iter().max().unwrap()).unwrap();
        (0..val).rev().filter_map(
            |i| self.get_index(i)
        ).next().unwrap_or(
            final_index
        )
    }

    fn simulate_once(self : &mut Self) {
        let picked_up_1 = self.take_next();
        let picked_up_2 = self.take_next();
        let picked_up_3 = self.take_next();
        let destination_index = self.destination();
        self.put_after(destination_index, picked_up_3);
        self.put_after(destination_index, picked_up_2);
        self.put_after(destination_index, picked_up_1);
        self.current_index += 1;
    }

    pub fn simulate(self : &mut Self, rounds : usize) {
        for _ in 0..rounds {
            self.simulate_once();
        }
    }

    fn get(self: &Self, index : usize) -> usize {
        self.items[index % self.items.len()]
    }

    fn get_index(self: &Self, elem : usize) -> Option<usize> {
        self.items.iter().position(|&x| x == elem)
    }

    pub fn labels(self: &Self, start : usize) -> String {
        let index = self.get_index(start).unwrap();
        (1..self.items.len()).map(
            |i| self.get(index + i).to_string()
        ).join("")
    }

    pub fn offset_labels(self: &Self, start : usize, offset : usize) -> usize {
        let index = self.get_index(start).unwrap();
        self.get(index + offset)
    }
}