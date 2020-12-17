use std::collections::HashSet;
use std::hash::Hash;

pub trait Position {
    fn surrounding(self : &Self) -> Vec<Self> 
      where Self : Sized;
    fn from_2d(i : i32, j :i32) -> Self;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position3D {
    i : i32, 
    j : i32,
    k : i32
}

impl Position for Position3D {
    fn surrounding(self : &Self) -> Vec<Position3D> {
        let mut positions : Vec<Position3D> = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if i != 0 || j != 0 || k != 0 {
                        positions.push(
                            Position3D {
                                i: self.i + i, 
                                j: self.j + j, 
                                k: self.k + k
                            }
                        );
                    }
                }
            }
        }
        positions
    }
    fn from_2d(i: i32, j : i32) -> Position3D {
        Position3D{ i : i, j : j, k : 0 }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position4D {
    i : i32, 
    j : i32,
    k : i32,
    w : i32
}

impl Position for Position4D {
    fn surrounding(self : &Self) -> Vec<Position4D> {
        let mut positions : Vec<Position4D> = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for w in -1..=1 {
                        if i != 0 || j != 0 || k != 0 || w != 0 {
                            positions.push(
                                Position4D {
                                    i: self.i + i, 
                                    j: self.j + j, 
                                    k: self.k + k,
                                    w: self.w + w
                                }
                            );
                        }
                    }
                }
            }
        }
        positions
    }
    fn from_2d(i: i32, j : i32) -> Position4D {
        Position4D{ i : i, j : j, k : 0, w: 0 }
    }
}

pub struct Conway<P> {
    cubes : HashSet<P>
}

impl<P> Conway<P> 
  where P : Position + Eq + Hash + Copy
{
    pub fn from_lines<I>(lines : I) -> Conway<P>
      where I : Iterator<Item = String> 
    {
        let mut cubes : HashSet<P> = HashSet::new();
        for (i, line) in lines.enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cubes.insert(P::from_2d(i as i32, j as i32));
                }
            }
        }
        Conway { cubes : cubes }
    }

    fn surrounding_occupied(self : &Self, cube : &P) -> usize {
        cube.surrounding().iter().filter(
            |surrounding_cube| self.cubes.contains(surrounding_cube)
        ).count()
    }

    fn potential_cubes(self : &Self) -> HashSet<P> {
        let mut potential_cubes : HashSet<P> = HashSet::new();
        for cube in self.cubes.iter() {
            for surrounding_cube in cube.surrounding() {
                potential_cubes.insert(surrounding_cube);
            }
            potential_cubes.insert(*cube);
        }
        potential_cubes
    }

    fn simulate_once(self : &mut Self) {
        let mut new_cubes : HashSet<P> = HashSet::new();
        for cube in self.potential_cubes().iter() {
            let surrounding_count = self.surrounding_occupied(cube);
            if self.cubes.contains(cube) {
                if surrounding_count == 2 || surrounding_count==3 {
                    new_cubes.insert(*cube);
                }
            } else {
                if surrounding_count==3 {
                    new_cubes.insert(*cube);
                }
            }
        }
        self.cubes = new_cubes;
    }

    pub fn num_cubes(self : &Self) -> usize {
        self.cubes.len()
    }

    pub fn simulate_n(self : &mut Self, num : usize) {
        for _ in 0..num {
            self.simulate_once();
        }
    }
}