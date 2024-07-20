// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move { x: i32, y: i32 },
    ChangeColor { r: u8, g: u8, b: u8 },
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor { r: 255, g: 100, b: 50 });
}