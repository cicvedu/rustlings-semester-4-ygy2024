// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.



pub fn animal_habitat(animal: &str) -> &'static str {
    // 定义一个枚举来统一不同类型的标识符
    #[derive(Debug, PartialEq)]
    enum AnimalId {
        Crab,
        Gopher,
        Snake,
        Unknown,
    }

    let identifier = match animal {
        "crab" => AnimalId::Crab,
        "gopher" => AnimalId::Gopher,
        "snake" => AnimalId::Snake,
        _ => AnimalId::Unknown,
    };

    // 使用 match 而不是 if 来处理枚举
    let habitat = match identifier {
        AnimalId::Crab => "Beach",
        AnimalId::Gopher => "Burrow",
        AnimalId::Snake => "Desert",
        AnimalId::Unknown => "Unknown",
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow");
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert");
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach");
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown");
    }
}