use pz6;

#[test]
fn test_snippet_naming_integration() {
    let snippet_name = "database_config";
    let result = pz6::format_filename(snippet_name);
    assert!(result.contains("database_config"));
    assert!(result.ends_with(".txt"));
}
