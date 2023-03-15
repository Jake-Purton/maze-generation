use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    visited: bool,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

impl Cell {
    fn new() -> Self {
        Self { visited: false, top: true, right: true, bottom: true, left: true }
    }
}

#[derive(Clone, Copy)]
pub enum WallDirection {
    Top,
    Right,
    Bottom,
    Left,
}

pub struct Map {
    map: Vec<Vec<Cell>>,
    size: usize,
}

impl Map {
    pub fn new(size: usize) -> Self {
        Self { map: vec![vec![Cell::new(); size]; size], size }
    }
    fn right(&self, x: usize, y: usize) -> Option<Cell> {
        if self.size <= x + 1  {
            None
        } else {
            Some(self.map[y][x + 1])
        }
    }
    fn left(&self, x: usize, y: usize) -> Option<Cell> {
        if x == 0  {
            None
        } else {
            Some(self.map[y][x - 1])
        }
    }
    fn top(&self, x: usize, y: usize) -> Option<Cell> {
        if y == 0  {
            None
        } else {
            Some(self.map[y - 1][x])
        }
    }
    fn bottom(&self, x: usize, y: usize) -> Option<Cell> {
        if self.size <= y + 1  {
            None
        } else {
            Some(self.map[y + 1][x])
        }
    }

    fn has_unvisited_neighbors (&self, x: usize, y: usize) -> Option<Vec<(usize, usize, WallDirection)>> {

        let mut vec = Vec::new();

        if let Some(cell) = self.left(x, y) {
            if !cell.visited {
                vec.push((x - 1, y, WallDirection::Left));
            }
        }
        if let Some(cell) = self.top(x, y) {
            if !cell.visited {
                vec.push((x, y - 1, WallDirection::Top));
            }
        }
        if let Some(cell) = self.right(x, y) {
            if !cell.visited {
                vec.push((x + 1, y, WallDirection::Right));
            }
        }
        if let Some(cell) = self.bottom(x, y) {
            if !cell.visited {
                vec.push((x, y + 1, WallDirection::Bottom));
            }
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }
}

pub fn depth_first_search (
    map: &mut Map,
    x: usize,
    y: usize,
    wall: Option<WallDirection>,
) {
    map.map[y][x].visited = true;

    if let Some(wall) = wall {
        match wall {
            WallDirection::Top => map.map[y][x].top = false,
            WallDirection::Right => map.map[y][x].right = false,
            WallDirection::Bottom => map.map[y][x].bottom = false,
            WallDirection::Left => map.map[y][x].left = false,
        }
    }

    while let Some(neighbors) = map.has_unvisited_neighbors(x, y) {

        let neighbor = neighbors[rand::thread_rng().gen_range(0..neighbors.len())];
        depth_first_search(map, neighbor.0, neighbor.1, Some(neighbor.2))

    }
}