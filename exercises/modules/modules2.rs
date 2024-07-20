// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.



mod delicious_snacks {
    // 确保 PEAR 和 CUCUMBER 常量在它们各自的模块中是公开的
    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    // 使用 pub use 来重新导出这些常量
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}