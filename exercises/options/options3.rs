// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // 使用 ref 来借用
        None => println!("No point provided."),
    }
    // 这里不需要 drop(y)，因为 y 没有被移动
}