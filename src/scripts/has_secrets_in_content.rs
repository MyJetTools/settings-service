pub fn has_secrets_in_content(content: &str) -> bool {
    content.contains("${")
}
