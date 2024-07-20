// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 修改为传递不可变引用

    string_uppercase(&data); // 接收一个不可变引用
}

// 修改为接收不可变引用
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// 修改为接收不可变引用，克隆数据并转换为大写
fn string_uppercase(data: &str) {
    let uppercased_data = data.to_uppercase();

    println!("{}", uppercased_data);
}