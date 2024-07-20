// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let vec0 = Vec::new();

    // 这里需要克隆vec0，因为fill_vec会获取vec0的所有权
    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // 这里不需要再次克隆，因为参数已经是一个值

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // 确保返回vec
}