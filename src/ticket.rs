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