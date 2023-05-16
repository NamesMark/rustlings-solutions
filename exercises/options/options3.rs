// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
    // ok, it was funny as I spent quite some time learning why I can't return just any other type from the main() function, what is Termination, etc., yet I just had to not touch it; good times
}
