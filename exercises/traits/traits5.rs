// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.



pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

// 定义一个新的 trait，它组合了 SomeTrait 和 OtherTrait
pub trait CombinedTrait: SomeTrait + OtherTrait {}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl CombinedTrait for SomeStruct {}

impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}
impl CombinedTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: CombinedTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}