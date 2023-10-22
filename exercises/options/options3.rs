// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ref y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y{
        Some(p) => println!("{}{}",p.x,p.y),
        _ => println!("weq"),
    }

    // if let Some(ref p)=y {
    //     println!("Co-ordinates are {},{} ", p.x, p.y);
    // }
    // else{
    //     panic!("no match!");
    // }
    y; // Fix without deleting this line.
}
