use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    pub row: u32,
    pub col: u32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
struct Cell {
    pub pos: Point,
    pub links: Vec<Point>
}

impl Cell {
    pub fn new(row:u32, col:u32) -> Cell {
        Cell{pos:Point{row, col}, links:vec![]}
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
        for l in &self.links {
            write!(f, " {} ", l);
        }
        write!(f, ")")
    }
}


fn main() {

    let mut a = Cell::new(0, 0);
    println!("a = {:?}", a,);

    let mut b = Cell::new(1, 1);
    let mut c = Cell::new(2, 2);
    a.link(&b);
    a.link(&c);
    b.bilink(&mut a);

    println!("Hello, world!");
    println!("a = {}, b = {}", a, b);
    a.unlink(&c);
    println!("a = {} b = {}", a, b);
    a.biunlink(&mut b);
    println!("a = {} b = {}", a, b);

}
