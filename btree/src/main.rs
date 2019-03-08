mod maze;

fn main() {

//    let mut a = maze::Cell::new(0, 0);
//    println!("a = {:?}", a,);

//    let mut b = maze::Cell::new(1, 1);
//    let mut c = maze::Cell::new(2, 2);
//    a.link(&b);
//    a.link(&c);
//    b.bilink(&mut a);

//    println!("Hello, world!");
//    println!("a = {}, b = {}", a, b);
//    a.unlink(&c);
//    println!("a = {} b = {}", a, b);
//    a.biunlink(&mut b);
//    println!("a = {} b = {}", a, b);

    let mut g = maze::Grid::new(4,4);
    println!("g={:?}", g);

    let r1 = g.random_point();
    println!("r1 = {}", r1);

    let r2 = g.random_point();
    println!("r1 = {}", r2);

    let r3 = g.random_point();
    println!("r1 = {}", r3);

    for c in g {
        println!("* {}", c);
    }

//    let d = &g[2][3];
//    print!("d = {}", d);

}
