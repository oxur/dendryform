//! # dendryform CLI
//!
//! Command-line interface for rendering software architecture diagrams
//! from declarative YAML/JSON definitions.
//!
//! ## Usage
//!
//! ```text
//! dendryform render architecture.yaml -o architecture.html
//! ```

fn main() {
    println!("dendryform {}", dendryform_core::version());
}
