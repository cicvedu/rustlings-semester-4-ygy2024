// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



fn main() {
    // 使用 is_none() 检查 Option
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Handle the case where my_option is None.
        // Since there's no specific action here, you could remove the if statement.
    }

    // 修复数组定义中的语法错误
    let my_arr = [
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 clear() 方法清空 vector
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // Vector is now empty
    println!("This Vec is empty, see? {:?}", my_vec);

    // 使用 std::mem::swap 交换 value_a 和 value_b
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}