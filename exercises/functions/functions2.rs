// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(3);
}

fn call_me(num: i32) { // 指定参数类型为 i32
    for i in 1..num { // 从 1 开始计数
        println!("Ring! Call number {}", i);
    }
}