fn welcome_message() -> String {
    "Hello, world!".into()
}

fn main() {
    println!("{}", welcome_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(welcome_message(), "Hello, world!".to_string());
    }
}
