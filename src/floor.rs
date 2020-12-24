use std::collections::HashSet;

enum Direction {
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

static ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::West,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::SouthEast,
    Direction::SouthWest
];

impl Direction {
    pub fn from_iter<I>(iter : &mut I) -> Option<Direction>
      where I : Iterator<Item = char>
    {
        match iter.next() {
            Some('e') => Some(Direction::East),
            Some('w') => Some(Direction::West),
            Some('n') => {
                match iter.next() {
                    Some('e') => Some(Direction::NorthEast),
                    Some('w') => Some(Direction::NorthWest),
                    _ => None
                }
            },
            Some('s') => {
                match iter.next() {
                    Some('e') => Some(Direction::SouthEast),
                    Some('w') => Some(Direction::SouthWest),
                    _ => None
                }
            },
            _ => None
        }
    }

    fn apply(self : &Self, pos : &(i64, i64)) -> (i64, i64) {
        let delta = self.delta();
        (pos.0 + delta.0, pos.1 + delta.1)
    }

    fn delta(self : &Self) -> (i64, i64) {
        match self {
            Direction::East => (0,1),
            Direction::West => (0,-1),
            Direction::NorthEast => (1,0),
            Direction::NorthWest => (1,-1),
            Direction::SouthEast => (-1,1),
            Direction::SouthWest => (-1,0),
        }
    }
}

pub struct Path {
    directions : Vec<Direction>
}

impl Path {
    pub fn from_line(string : &str) -> Path {
        let mut directions : Vec<Direction> = Vec::new();
        let mut chars = string.chars();
        while let Some(dir) = Direction::from_iter(&mut chars) {
            directions.push(dir);
        }
        Path {
            directions : directions
        }
    }

    fn end_point(self : &Self) -> (i64, i64) {
        let mut pos : (i64, i64) = (0,0);
        for (delta_x, delta_y) in self.directions.iter().map(|dir| dir.delta()) {
            pos.0 += delta_x;
            pos.1 += delta_y;
        }
        pos
    }
}
 

pub struct Floor {
    black_tiles : HashSet<(i64, i64)>
}

fn adjacent_tiles(pos : &(i64,i64)) -> Vec<(i64, i64)> {
    ALL_DIRECTIONS.iter().map(
        |dir| dir.apply(pos)
    ).collect()
}

impl Floor {
    pub fn new() -> Floor {
        Floor {
            black_tiles : HashSet::new()
        }
    }

    pub fn apply_paths(self : &mut Self,paths : &Vec<Path>) {
        for path in paths.iter() {
            self.flip(path.end_point())
        }
    }

    pub fn len(self : &Self) -> usize {
        self.black_tiles.len()
    }

    pub fn simulate(self : &mut Self, days : usize) {
        for i in 0..days {
            println!("Day {}: {}", i, self.len());
            self.simulate_once();
        }
    }

    fn simulate_once(self : &mut Self) {
        let mut to_flip : Vec<(i64,i64)> = Vec::new();
        for tile in self.possible_tiles() {
            let num = self.number_adjacent(&tile);
            if self.black_tiles.contains(&tile) {
                if num == 0 || num > 2 {
                    to_flip.push(tile);
                }
            } else {
                if num == 2 {
                    to_flip.push(tile);
                }
            }
        }
        for tile in to_flip {
            self.flip(tile);
        }
    }

    fn number_adjacent(self : &Self, pos : &(i64, i64)) -> usize {
        adjacent_tiles(pos).into_iter().filter(
            |tile| self.black_tiles.contains(tile)
        ).count()
    }

    fn possible_tiles(self : &Self) -> HashSet<(i64, i64)> {
        let mut possible : HashSet<(i64, i64)> = HashSet::new();
        for black_tile in self.black_tiles.iter() {
            possible.insert(*black_tile);
            for adjacent_tile in adjacent_tiles(black_tile) {
                possible.insert(adjacent_tile);
            }
        }
        possible
    }

    fn flip(self : &mut Self, pos : (i64, i64)) {
        if self.black_tiles.contains(&pos) {
            self.black_tiles.remove(&pos);
        } else {
            self.black_tiles.insert(pos);
        }
    }
}