// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 if let 进行模式匹配
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None; range]; // 初始化为 None 的向量

        for i in 1..(range + 1) {
            optional_integers.push(Some(i as i8)); // 填充向量
        }

        let mut cursor = range;

        // 使用 while let 进行模式匹配
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor as i8);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}