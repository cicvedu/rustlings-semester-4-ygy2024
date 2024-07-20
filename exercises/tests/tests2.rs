// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 使测试通过的例子
        let x = 5;
        let y = 5;
        assert_eq!(x, y);

        // 使测试失败的例子（取消注释下面的代码）
        // let x = 5;
        // let y = 3;
        // assert_eq!(x, y);
    }
}