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