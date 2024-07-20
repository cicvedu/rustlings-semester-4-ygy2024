// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        // 要使测试编译，我们需要给 assert! 提供一个布尔表达式。
        // 要使测试通过，这个表达式需要是 true。
        // 要使测试失败，这个表达式需要是 false。

        // 使测试通过的例子
        assert!(true);

        // 使测试失败的例子（注释掉上面的测试，取消注释下面的测试）
        // assert!(false);
    }
}