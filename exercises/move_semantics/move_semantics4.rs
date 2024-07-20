// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // 这里不需要使用vec0，因为fill_vec现在会在内部创建Vec
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 修改fill_vec函数，使其不再接收参数，而是在内部创建并返回Vec<i32>
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // 创建一个新的Vec<i32>

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // 返回创建的Vec
}