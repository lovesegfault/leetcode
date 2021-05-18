use std::collections::HashSet;

type Map = Vec<Vec<bool>>;

pub fn count_islands2(map: &Map) -> u64 {
    let mut islands = 0;
    let mut neighbors: Vec<(usize, usize)> =
        Vec::with_capacity(map.len() * map.get(0).map(|r| r.len()).unwrap_or(1));
    let mut visited = std::collections::HashSet::<(usize, usize)>::new();

    let (height, width) = (map.len(), map.get(0).map_or(0, Vec::len));

    for y in 0..height {
        for x in 0..width {
            if visited.contains(&(x, y)) {
                continue;
            }
            islands += 1;
            visited.insert((x, y));
            neighbors.push((x, y));
            while let Some((x, y)) = neighbors.pop() {
                let mut process_neighbor = |(x, y)| {
                    let maybe_cell = map.get(y).and_then(|row: &Vec<bool>| row.get(x));
                    if let Some(true) = maybe_cell {
                        neighbors.push((x, y));
                        visited.insert((x, y));
                    }
                };
                // each square has four neighbors we care about following
                // left neighbor
                process_neighbor((x - 1, y));
                // right neighbor
                process_neighbor((x + 1, y));
                // top neighbor
                process_neighbor((x, y - 1));
                // bottom neighbor
                process_neighbor((x, y + 1));
            }
            neighbors.clear();
        }
    }

    islands
}

pub fn num_islands(mut map: Vec<Vec<char>>) -> i32 {
    let mut islands = 0;
    let mut neighbors = Vec::with_capacity(map.len() * map.get(0).map_or(0, Vec::len));

    let (height, width) = (map.len(), map.get(0).map_or(0, Vec::len));

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '0' {
                continue;
            }
            // mark the current spot as visited
            map[y][x] = '0';
            // we've found an island
            islands += 1;
            // discover the extent of said island
            neighbors.push((x, y));
            while let Some((x, y)) = neighbors.pop() {
                let mut process_neighbor = |(x, y)| {
                    let maybe_cell = map
                        .get_mut(y)
                        .and_then(|row: &mut Vec<char>| row.get_mut(x));
                    if let Some(cell @ '1') = maybe_cell {
                        neighbors.push((x, y));
                        *cell = '0'
                    }
                };
                // each square has four neighbors we care about following
                // left neighbor
                process_neighbor((x - 1, y));
                // right neighbor
                process_neighbor((x + 1, y));
                // top neighbor
                process_neighbor((x, y - 1));
                // bottom neighbor
                process_neighbor((x, y + 1));
            }
            neighbors.clear();
        }
    }

    islands
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
    Halt,
}

#[derive(Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn walk(&self, to: &Movement) -> Self {
        use Movement::*;
        match to {
            Left => Self(self.0 - 1, self.1),
            Right => Self(self.0 + 1, self.1),
            Up => Self(self.0, self.1 - 1),
            Down => Self(self.0, self.1 + 1),
            Halt => *self,
        }
    }
}

pub fn num_unique_islands(mut map: Vec<Vec<i32>>) -> i32 {
    let max_island_size = map.len() * map.get(0).map_or(0, Vec::len);
    let mut unique_islands: HashSet<Vec<Movement>> = HashSet::with_capacity(max_island_size);
    let mut signature: Vec<Movement> = Vec::with_capacity(max_island_size);
    let mut neighbors: Vec<Position> = Vec::with_capacity(max_island_size);

    let (height, width) = (map.len(), map.get(0).map_or(0, Vec::len));

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                continue;
            }
            // mark the current spot as visited
            map[y][x] = 0;
            // discover the extent of said island
            neighbors.push(Position::new(x, y));
            while let Some(position) = neighbors.pop() {
                let mut process_neighbor = |pos: &Position, mov: Movement| {
                    let pos = pos.walk(&mov);
                    let maybe_cell = map
                        .get_mut(pos.y())
                        .and_then(|row: &mut Vec<_>| row.get_mut(pos.x()));
                    if let Some(cell @ 1) = maybe_cell {
                        // add this neighbor as a point to look around
                        neighbors.push(pos);
                        // and mark it as visited
                        *cell = 0;
                        // add this movement to the current path signature
                        signature.push(mov);
                    }
                };
                // each square has four neighbors we care about following
                // left neighbor
                process_neighbor(&position, Movement::Left);
                // right neighbor&
                process_neighbor(&position, Movement::Right);
                // top neighbor
                process_neighbor(&position, Movement::Up);
                // bottom neighbo&r
                process_neighbor(&position, Movement::Down);
                signature.push(Movement::Halt);
            }
            unique_islands.insert(signature.clone());
            signature.clear();
            neighbors.clear();
        }
    }

    unique_islands.len() as i32
}

fn main() {
    println!("Hello, world!");
}
