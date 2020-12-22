use std::collections::HashMap;
use std::collections::HashSet;

struct Tile {
    id : u64,
    data : Vec<Vec<bool>>
}

pub struct TileSet {
    normalized_edge_counts : HashMap<Vec<bool>, Vec<u64>>,
    tiles : HashMap<u64, Tile>
}

enum Side {
    Top,
    Bottom,
    Left,
    Right
}

pub struct TileSolution<'a> {
    size : usize,
    tileset : &'a TileSet,
    used : HashSet<u64>,
    tiles : HashMap<(i64, i64), Tile>
}

fn flip_edge(edge : &Vec<bool>) -> Vec<bool> {
    edge.iter().rev().cloned().collect()
}

fn normalized_edge(edge : &Vec<bool>) -> Vec<bool> {
    // Ineffiecient try all and chose largest
    let vars = vec!(
        edge.clone(),
        flip_edge(edge)
    );
    vars.into_iter().max().unwrap()
}

impl Tile {
    pub fn from_string(string : &str) -> Tile {
        let mut line_iter = string.lines();
        let id = line_iter.next().unwrap()
                          .strip_prefix("Tile ").unwrap()
                          .strip_suffix(":").unwrap()
                          .parse::<u64>().unwrap();
        let data : Vec<Vec<bool>> = line_iter.map(
            |line| line.chars().map(
                |ch| ch == '#'
            ).collect()
        ).collect();
        Tile { id : id, data : data }
    }

    pub fn edge(self : &Self, side : &Side) -> Vec<bool> {
        match side {
            Side::Top => self.data[0].clone(),
            Side::Bottom => self.data[9].clone(),
            Side::Left => self.data.iter().map(|line| line[0]).collect(),
            Side::Right => self.data.iter().map(|line| line[9]).collect()
        }
    } 

    pub fn edges(self : &Self) -> Vec<Vec<bool>> {
        let top = self.edge(&Side::Top);
        let bottom  = self.edge(&Side::Bottom);
        let left = self.edge(&Side::Left);
        let right = self.edge(&Side::Right);
        vec!(top, bottom, left, right)
    }

    pub fn normalized_edges(self : &Self) -> Vec<Vec<bool>> {
        self.edges().iter().map(
            |edge| normalized_edge(edge)
        ).collect()
    }

    pub fn varients(self : &Self) -> Vec<Tile> {
        let mut tiles : Vec<Tile> = vec!(self.flip_x().flip_x());
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles.push(
            self.flip_x()
        );
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles.push(
            tiles.last().unwrap().rotate()
        );
        tiles
    } 

    pub fn rotate(self : &Self) -> Tile {
        self.flip_xy().flip_x()
    } 

    pub fn flip_x(self : &Self) -> Tile {
        Tile {
            id : self.id,
            data : self.data.iter().map(
                |row| row.iter().rev().cloned().collect() 
            ).collect()
        }
    }

    pub fn flip_xy(self : &Self) -> Tile {
        Tile {
            id : self.id,
            data : (0..10).map(
                |i| {
                    (0..10).map(
                        |j| self.data[j][i]
                    ).collect()
                }
            ).collect()
        }
    }
}

impl TileSet {
    pub fn from_string(string : &str) -> TileSet {
        let tiles : HashMap<u64, Tile> = string.split("\n\n").map(
            |chunk| Tile::from_string(chunk)
        ).map(
            |tile| (tile.id, tile)
        ).collect();
        let mut normalized_edge_counts : HashMap<Vec<bool>, Vec<u64>> 
            = HashMap::new();
        for tile in tiles.values() {
            for edge in tile.normalized_edges() {
                normalized_edge_counts.entry(edge).or_insert(Vec::new()).push(tile.id);
            }
        }
        println!("{}", tiles.values().count());
        TileSet{
            tiles : tiles,
            normalized_edge_counts : normalized_edge_counts
        }
    }

    pub fn tile_class(self : &Self, class : usize) -> Vec<u64> {
        let mut edge_tiles_ids : Vec<u64> = self.tiles.values().filter(
            |tile| tile.normalized_edges().iter().filter(
                |&edge| self.normalized_edge_counts.get(edge).unwrap().iter().count() == 1
            ).count() >= class
        ).map(|tile| tile.id).collect();
        edge_tiles_ids.sort();
        edge_tiles_ids
    }

    fn is_edge_unmatched(self : &Self, edge : &Vec<bool>) -> bool {
        self.normalized_edge_counts.get(&normalized_edge(edge)).unwrap().iter().count() == 1
    } 

    fn unmatched_edges(self : &Self, tile_id : u64) -> Vec<Vec<bool>> {
        let tile = &self.tiles[&tile_id];
        tile.edges().into_iter().filter(
            |edge| self.is_edge_unmatched(edge)
        ).collect()
    }

    fn find_by_top_left(self : &Self, top : Option<Vec<bool>>, left : Option<Vec<bool>>, exclude : &HashSet<u64>) -> Tile {
        match (top,left) {
            (Some(top_edge), Some(left_edge)) => self.find_by_top_left_edges(&top_edge, &left_edge, exclude).unwrap(),
            (Some(top_edge), None) => self.find_left_edge(&top_edge, exclude),
            (None, Some(left_edge)) => self.find_top_edge(&left_edge, exclude),
            (None, None) => self.find_top_left_corner(exclude)
        }
    }

    fn find_top_left_corner(self : &Self, exclude : &HashSet<u64>) -> Tile {
        let tile_id = self.tile_class(2)[0];
        let unmatched_edges = self.unmatched_edges(tile_id);
        self.find_by_top_left_edges(&unmatched_edges[0], &unmatched_edges[1], exclude).or(
            self.find_by_top_left_edges(&unmatched_edges[0], &flip_edge(&unmatched_edges[1]), exclude)
        ).or(
            self.find_by_top_left_edges(&flip_edge(&unmatched_edges[0]), &unmatched_edges[1], exclude)
        ).or(
            self.find_by_top_left_edges(&flip_edge(&unmatched_edges[0]), &flip_edge(&unmatched_edges[1]), exclude)
        ).unwrap()
    }

    

    fn find_top_edge(self : &Self, left : &Vec<bool>, exclude : &HashSet<u64>) -> Tile {
        for edge_tile_id in self.tile_class(1) {
            for unmatched_edge in self.unmatched_edges(edge_tile_id) {
                let tile = self.find_by_top_left_edges(&unmatched_edge, left, exclude).or(
                    self.find_by_top_left_edges(&flip_edge(&unmatched_edge), left, exclude)
                );
                if tile.is_some() {
                    return tile.unwrap()
                }
            }
        }
        unreachable!();
    }

    fn find_left_edge(self : &Self, top : &Vec<bool>, exclude : &HashSet<u64>) -> Tile {
        for edge_tile_id in self.tile_class(1) {
            for unmatched_edge in self.unmatched_edges(edge_tile_id) {
                let tile = self.find_by_top_left_edges(top, &unmatched_edge, exclude).or(
                    self.find_by_top_left_edges(top, &flip_edge(&unmatched_edge), exclude)
                );
                if tile.is_some() {
                    return tile.unwrap()
                }
            }
        }
        unreachable!();
    }

    fn find_by_top_left_edges(self : &Self, top : &Vec<bool>, left : &Vec<bool>, exclude : &HashSet<u64>) -> Option<Tile> {
        for tile in self.tiles.values() {
            if !exclude.contains(&tile.id) {
                for var in tile.varients() {
                    if var.edge(&Side::Top) == *top && var.edge(&Side::Left) == *left {
                        return Some(var);
                    }
                }
            }
        }
        None
    }


    pub fn solve<'a>(self : &'a Self, size : usize) -> TileSolution<'a> {
        let mut solution = TileSolution::new(&self, size);
        solution.solve();
        solution
    }
}

impl<'a> TileSolution<'a> {
    pub fn new<'b>(tileset : &'b TileSet, size : usize) -> TileSolution<'b> {
        TileSolution {
            size : size,
            used : HashSet::new(),
            tileset : tileset,
            tiles : HashMap::new()
        }
    }

    pub fn solve(self : &mut Self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.solve_tile(i as i64, j as i64);
            }
        }
    }

    pub fn corner_tiles(self : &Self) -> Vec<u64> {
        vec!(
            self.tiles[&(0, 0)].id,
            self.tiles[&(0, self.size as i64-1)].id,
            self.tiles[&(self.size as i64-1, 0)].id,
            self.tiles[&(self.size as i64-1, self.size as i64-1)].id
        )
    }

    fn solve_tile(self : &mut Self, i : i64, j : i64) {
        let top = self.tiles.get(&(i-1,j)).map(
            |tile| tile.edge(&Side::Bottom)
        );
        let left = self.tiles.get(&(i,j-1)).map(
            |tile| tile.edge(&Side::Right)
        );
        let tile = self.tileset.find_by_top_left(top, left, &self.used);
        self.used.insert(tile.id);
        println!("{} {} - done => {}", i, j, tile.id);
        self.tiles.insert((i,j), tile);
    }
}