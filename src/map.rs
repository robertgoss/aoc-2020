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