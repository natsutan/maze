mod maze;
mod binary_tree;

fn main() {


    let mut grid = maze::Grid::new(4, 4);
    println!("g={:?}", grid);

    binary_tree::make(&mut grid);
    println!("g={:?}", grid);

}
