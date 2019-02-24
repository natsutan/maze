use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub row: u32,
    pub col: u32
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



}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, links = ", self.pos);

//        write!(f, "N:{} S:{} E:{} W:{}= ", self.north, self.south, self.east, self.west);


        for l in &self.links {
            write!(f, " {} ", l);
        }
        write!(f, ")")
    }
}
