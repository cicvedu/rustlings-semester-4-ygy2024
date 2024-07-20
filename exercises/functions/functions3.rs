// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me();
}

// 移除参数，使 call_me 函数可以接受调用而不需要参数
fn call_me() {
    let num = 5; // 指定一个固定的数字
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}