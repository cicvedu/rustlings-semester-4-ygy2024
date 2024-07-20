// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + c.as_str()
        }
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut v = Vec::new();
    for word in words {
        v.push(capitalize_first(word));
    }
    v
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut v = Vec::new();
    for word in words {
        if !word.trim().is_empty() { // Check if the word is not empty after trimming whitespace
            v.push(capitalize_first(word));
        }
    }
    v.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), vec!["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}