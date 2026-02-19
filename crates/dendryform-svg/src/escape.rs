//! XML text escaping for SVG content.

/// Escapes special XML characters in text content.
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml_ampersand() {
        assert_eq!(escape_xml("A & B"), "A &amp; B");
    }

    #[test]
    fn test_escape_xml_angle_brackets() {
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
    }

    #[test]
    fn test_escape_xml_quotes() {
        assert_eq!(escape_xml(r#"say "hello""#), "say &quot;hello&quot;");
    }

    #[test]
    fn test_escape_xml_apostrophe() {
        assert_eq!(escape_xml("it's"), "it&apos;s");
    }

    #[test]
    fn test_escape_xml_no_special_chars() {
        assert_eq!(escape_xml("hello world"), "hello world");
    }

    #[test]
    fn test_escape_xml_all_special() {
        assert_eq!(
            escape_xml(r#"<a & 'b' "c">"#),
            "&lt;a &amp; &apos;b&apos; &quot;c&quot;&gt;"
        );
    }
}
