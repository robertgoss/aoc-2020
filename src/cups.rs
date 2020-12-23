struct CycleData {
    next_val : Vec<usize>
}

impl CycleData {
    fn new(size : usize) -> CycleData {
        let mut data : Vec<usize> = (1..=size).collect();
        data[size-1] = 0;
        CycleData {
            next_val : data
        }
    }

    fn set_next(self : &mut Self, val : usize, val_next : usize) {
        self.next_val[val-1] = val_next-1;
    }

    fn get_next(self : &Self, val : usize) -> usize {
        self.next_val[val-1]+1
    }

    fn add_after(self : &mut Self, val : usize, val_add : usize) {
        let curr_next = self.next_val[val-1];
        self.next_val[val-1] = val_add-1;
        self.next_val[val_add-1] = curr_next;
    }

    fn remove_next(self : &mut Self, val : usize) -> usize {
        let next = self.next_val[val-1];
        self.next_val[val-1] = self.next_val[next];
        next + 1
    }
}

pub struct Cups {
    current_val : usize,
    size : usize,
    data : CycleData
}

impl Cups {
    pub fn from_cycle_ints(ints : Vec<usize>) -> Cups {
        let mut data = CycleData::new(ints.len());
        for window in ints.windows(2) {
            data.set_next(window[0], window[1]);
        }
        data.set_next(ints[ints.len()-1], ints[0]);
        Cups {
            current_val : ints[0],
            size : ints.len(),
            data : data
        }
    }

    fn simulate_once(self : &mut Self) {
        let taken_1 = self.data.remove_next(self.current_val);
        let taken_2 = self.data.remove_next(self.current_val);
        let taken_3 = self.data.remove_next(self.current_val);
        let destination_val = self.destination_val(vec!(taken_1, taken_2, taken_3));
        self.data.add_after(destination_val, taken_3);
        self.data.add_after(destination_val, taken_2);
        self.data.add_after(destination_val, taken_1);
        self.current_val = self.data.get_next(self.current_val);
    }

    fn destination_val(self : &Self, invalid : Vec<usize>) -> usize {
        let mut destination = self.decrement(self.current_val);
        while invalid.contains(&destination) {
            destination = self.decrement(destination);
        }
        destination
    }

    fn decrement(self : &Self, val : usize) -> usize {
        if val <= 1 {
            self.size
        } else {
            val - 1
        }
    }


    pub fn simulate(self : &mut Self, rounds : usize) {
        for _ in 0..rounds {
            self.simulate_once();
        }
    }

    pub fn labels(self: &Self, start : usize) -> String {
        let mut vals : Vec<String> = Vec::new();
        let mut val = self.data.get_next(start);
        while val != 1 {
            vals.push(val.to_string());
            val = self.data.get_next(val);
        }
        vals.join("")
    }

    pub fn offset_labels(self : &Self, start : usize, offset : usize) -> usize {
        let mut val = start;
        for _ in 0..offset {
            val = self.data.get_next(val);
        }
        val
    }
}