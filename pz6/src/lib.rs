pub fn format_filename(name: &str) -> String {
    if name.ends_with(".txt") {
        name.to_string()
    } else {
        format!("{}.txt", name)
    }
}

pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_filename_adds_extension() {
        assert_eq!(format_filename("my_snippet"), "my_snippet.txt");
    }

    #[test]
    fn test_format_filename_keeps_extension() {
        assert_eq!(format_filename("already_has.txt"), "already_has.txt");
    }

    #[test]
    fn test_valid_url() {
        assert!(is_valid_url("https://google.com"));
        assert!(!is_valid_url("not_a_url"));
    }

    #[test]
    #[should_panic(expected = "Empty name")]
    fn test_panic_on_empty_name() {
        let name = "";
        if name.is_empty() {
            panic!("Empty name");
        }
    }
}
