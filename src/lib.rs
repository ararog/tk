pub fn tk(size: i32) -> String {
    let mut random_string: String = String::from("");
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    for _i in 0..size {
        let random_index = rand::random_range(0..CHARACTERS.len());
        random_string = random_string + CHARACTERS.get(random_index..random_index + 1).unwrap();
    }

    String::from(random_string)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_size() {
        let code = tk(8);
        assert_eq!(code.len(), 8);
    }
}
