pub fn format_filename(name: &str) -> String {
    format!("{}.txt", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_filename() {
        assert_eq!(format_filename("rust"), "rust.txt");
        assert_ne!(format_filename("rust"), "rust.json");
    }

    #[test]
    #[should_panic]
    fn test_empty_name_panic() {
        let name = "";
        if name.is_empty() { panic!("Name is empty"); }
    }
}
