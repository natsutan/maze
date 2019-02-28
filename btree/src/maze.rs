use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub row: u32,
    pub col: u32
}

impl Point {
    pub fn new(row: i32, col: i32, max_row: u32, max_col: u32) -> Some(Point) {
        if 

        Point { pos: Point { row, col }, north: None, south: None, east: None, west: None, links: vec![] }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
pub struct Cell {
    pub pos: Point,
    pub north: Option<Point>,
    pub south: Option<Point>,
    pub east: Option<Point>,
    pub west: Option<Point>,
    pub links: Vec<Point>
}

impl Cell {
    pub fn new(row:u32, col:u32) -> Cell {
        Cell{pos:Point{row, col}, north:None, south:None, east:None, west:None, links:vec![]}
    }

    pub fn link(&mut self, cell:& Cell) {
        let pos =  cell.pos.clone();
        self.links.push(pos)
    }

    pub fn bilink(&mut self, cell:&mut Cell) {
        self.link(cell);
        cell.link(self)
    }

    pub fn unlink(&mut self, cell:& Cell) {
        let pos:Point = cell.pos.clone();
        self.links.retain(|&x| x != pos)
    }

    pub fn biunlink(&mut self, cell:&mut Cell) {
        self.unlink(cell);
        cell.unlink(self)
    }

    pub fn neighbors(self) ->Vec<Point>{
        let mut neighbors:Vec<Point> = vec![];
        if let Some(v) = self.north {
            neighbors.push(v);
        }
        if let Some(v) = self.south {
            neighbors.push(v);
        }
        if let Some(v) = self.east {
            neighbors.push(v);
        }
        if let Some(v) = self.west {
            neighbors.push(v);
        }

        neighbors
    }

}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, ", self.pos);
        if let Some(v) = self.north {
            write!(f, "N:{} ", v);
        }
        if let Some(v) = self.south {
            write!(f, "S:{} ", v);
        }
        if let Some(v) = self.east {
            write!(f, "E:{} ", v);
        }
        if let Some(v) = self.west {
            write!(f, "W:{} ", v);
        }

        if self.links.len() != 0 {
            write!(f, " links = ", );
            for l in &self.links {
                write!(f, "{} ", l);
            }
        } else {
            write!(f, "no link ");
        }

        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct Grid {
    pub row:u32,
    pub col:u32,
    pub grid:Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(row: u32, col: u32) -> Grid {
        let mut grid = Grid { row: row, col: col, grid: vec![] };

        for r in 0..row {
            let mut rvec: Vec<Cell> = vec![];
            for c in 0..col {
                let mut cell = Cell::new(r, c);
                cell.north = Some(Point { row: r - 1, col: c });
                cell.south = Some(Point { row: r + 1, col: c });
                cell.west = Some(Point { row: r, col: c - 1 });
                cell.east = Some(Point { row: r, col: c + 1 });

                println!("cell = {}", cell);
                rvec.push(cell);
            }
            grid.grid.push(rvec);
        }
        grid
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Cell>;
    fn index(& self, row: usize) -> &Vec<Cell> {
        & self.grid[row]
    }
}

impl IndexMut < usize > for Grid {
    fn index_mut( & mut self, row: usize) -> & mut Vec<Cell> {
        & mut self.grid[row]
    }
}