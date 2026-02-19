//! # dendryform-parse
//!
//! YAML and JSON parser for dendryform diagram definitions.
//!
//! Reads a diagram string or file, deserializes it into a validated
//! [`Diagram`](dendryform_core::Diagram). Validation (duplicate IDs,
//! dangling edges, empty tiers, nesting depth) happens automatically
//! during deserialization.
//!
//! ## Quick Start
//!
//! ```no_run
//! use dendryform_parse::parse_yaml_file;
//!
//! let diagram = parse_yaml_file("examples/taproot/taproot.yml").unwrap();
//! println!("Diagram: {}", diagram.header().title().text());
//! ```

mod error;

use std::path::Path;

use dendryform_core::Diagram;

pub use error::ParseError;

/// Parses a YAML string into a validated [`Diagram`].
pub fn parse_yaml(input: &str) -> Result<Diagram, ParseError> {
    let diagram: Diagram = serde_yml::from_str(input)?;
    Ok(diagram)
}

/// Parses a YAML file into a validated [`Diagram`].
pub fn parse_yaml_file(path: impl AsRef<Path>) -> Result<Diagram, ParseError> {
    let content = std::fs::read_to_string(path)?;
    parse_yaml(&content)
}

/// Parses a JSON string into a validated [`Diagram`].
pub fn parse_json(input: &str) -> Result<Diagram, ParseError> {
    let diagram: Diagram = serde_json::from_str(input)?;
    Ok(diagram)
}

/// Parses a JSON file into a validated [`Diagram`].
pub fn parse_json_file(path: impl AsRef<Path>) -> Result<Diagram, ParseError> {
    let content = std::fs::read_to_string(path)?;
    parse_json(&content)
}

/// Returns the version of the dendryform-parse crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert_eq!(version(), "0.0.1");
    }

    #[test]
    fn test_parse_taproot_yaml() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = parse_yaml(yaml).expect("taproot.yml should parse successfully");
        assert_eq!(diagram.header().title().text(), "system architecture");
        assert_eq!(diagram.header().title().accent(), "taproot");
        assert_eq!(
            diagram.header().subtitle(),
            "natural language analytics over BigQuery via MCP"
        );
        assert_eq!(diagram.header().theme(), "dark");
    }

    #[test]
    fn test_parse_taproot_yaml_layers() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = parse_yaml(yaml).unwrap();
        // taproot.yml has: tier, connector, tier, flow_labels, tier = 5 top-level layers
        assert_eq!(diagram.layers().len(), 5);
    }

    #[test]
    fn test_parse_taproot_yaml_edges() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = parse_yaml(yaml).unwrap();
        // taproot.yml has 18 edges
        assert_eq!(diagram.edges().len(), 18);
    }

    #[test]
    fn test_parse_taproot_yaml_legend() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = parse_yaml(yaml).unwrap();
        assert_eq!(diagram.legend().len(), 6);
    }

    #[test]
    fn test_parse_yaml_malformed() {
        let yaml = "this: is: not: valid: yaml: [";
        let err = parse_yaml(yaml).unwrap_err();
        assert!(matches!(err, ParseError::Yaml(_)));
        let msg = format!("{err}");
        assert!(msg.contains("YAML parse error"));
    }

    #[test]
    fn test_parse_yaml_missing_required_field() {
        let yaml = r#"
diagram:
  title:
    text: "test"
    accent: "test"
  subtitle: "test"
layers: []
edges: []
"#;
        // Missing `theme` field in diagram header
        let err = parse_yaml(yaml).unwrap_err();
        assert!(matches!(err, ParseError::Yaml(_)));
    }

    #[test]
    fn test_parse_yaml_duplicate_node_id() {
        let yaml = r#"
diagram:
  title:
    text: "test"
    accent: "t"
  subtitle: "sub"
  theme: dark
layers:
  - tier:
      id: main
      nodes:
        - id: app
          kind: system
          color: blue
          icon: "◇"
          title: "App"
          description: "The app"
        - id: app
          kind: system
          color: blue
          icon: "◇"
          title: "App duplicate"
          description: "Same ID"
"#;
        let err = parse_yaml(yaml).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("duplicate node ID"));
    }

    #[test]
    fn test_parse_yaml_dangling_edge() {
        let yaml = r#"
diagram:
  title:
    text: "test"
    accent: "t"
  subtitle: "sub"
  theme: dark
layers:
  - tier:
      id: main
      nodes:
        - id: app
          kind: system
          color: blue
          icon: "◇"
          title: "App"
          description: "The app"
edges:
  - from: app
    to: ghost
    kind: uses
"#;
        let err = parse_yaml(yaml).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("ghost"));
    }

    #[test]
    fn test_parse_json_round_trip() {
        let yaml = include_str!("../../../examples/taproot/taproot.yml");
        let diagram = parse_yaml(yaml).unwrap();
        let json = serde_json::to_string_pretty(&diagram).unwrap();
        let from_json = parse_json(&json).unwrap();
        assert_eq!(diagram, from_json);
    }

    #[test]
    fn test_parse_json_malformed() {
        let err = parse_json("{invalid json").unwrap_err();
        assert!(matches!(err, ParseError::Json(_)));
        let msg = format!("{err}");
        assert!(msg.contains("JSON parse error"));
    }

    #[test]
    fn test_parse_error_io() {
        let err = parse_yaml_file("/nonexistent/path/to/file.yml").unwrap_err();
        assert!(matches!(err, ParseError::Io(_)));
        let msg = format!("{err}");
        assert!(msg.contains("I/O error"));
    }

    #[test]
    fn test_parse_error_display_and_source() {
        let err = parse_yaml("not valid yaml [").unwrap_err();
        // Verify Error trait source chain works
        let source = std::error::Error::source(&err);
        assert!(source.is_some());
    }

    #[test]
    fn test_parse_yaml_empty_tier_rejected() {
        let yaml = r#"
diagram:
  title:
    text: "test"
    accent: "t"
  subtitle: "sub"
  theme: dark
layers:
  - tier:
      id: empty
      nodes: []
"#;
        let err = parse_yaml(yaml).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("empty"));
    }

    #[test]
    fn test_parse_yaml_minimal_valid() {
        let yaml = r#"
diagram:
  title:
    text: "test"
    accent: "t"
  subtitle: "sub"
  theme: dark
layers:
  - tier:
      id: main
      nodes:
        - id: app
          kind: system
          color: blue
          icon: "◇"
          title: "App"
          description: "The app"
"#;
        let diagram = parse_yaml(yaml).unwrap();
        assert_eq!(diagram.header().title().text(), "test");
        assert_eq!(diagram.layers().len(), 1);
        assert!(diagram.edges().is_empty());
        assert!(diagram.legend().is_empty());
    }
}
