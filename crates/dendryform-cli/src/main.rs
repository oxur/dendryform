//! # dendryform CLI
//!
//! Command-line interface for rendering software architecture diagrams
//! from declarative YAML/JSON definitions.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use dendryform_core::Theme;
use dendryform_html::render_html;
use dendryform_layout::compute_layout;
use dendryform_parse::{ParseError, parse_yaml_file};
use dendryform_png::render_png;
use dendryform_svg::render_svg;

/// dendryform — render architecture diagrams from YAML
#[derive(Parser)]
#[command(name = "dendryform", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Render a diagram file to HTML, SVG, or PNG
    Render {
        /// Input YAML file
        input: PathBuf,
        /// Output file path (defaults to input stem + format extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme name or path (default: dark)
        #[arg(long, default_value = "dark")]
        theme: String,
        /// Output format: html, svg, or png
        #[arg(short, long, default_value = "html")]
        format: String,
        /// SVG viewport width in pixels (used with svg and png formats)
        #[arg(long, default_value = "1100")]
        width: f32,
        /// Scale factor for PNG output (1.0 = 1x, 2.0 = retina)
        #[arg(long, default_value = "1.0")]
        scale: f32,
    },
    /// Validate a diagram file without rendering
    Validate {
        /// Input YAML file
        input: PathBuf,
    },
    /// Generate a starter YAML diagram to stdout
    Init,
    /// List available built-in themes
    Themes,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Render {
            input,
            output,
            theme: theme_name,
            format,
            width,
            scale,
        } => cmd_render(
            &input,
            output.as_deref(),
            &theme_name,
            &format,
            width,
            scale,
        ),
        Command::Validate { input } => cmd_validate(&input),
        Command::Init => cmd_init(),
        Command::Themes => cmd_themes(),
    }
}

fn cmd_render(
    input: &PathBuf,
    output: Option<&std::path::Path>,
    theme_name: &str,
    format: &str,
    width: f32,
    scale: f32,
) -> ExitCode {
    let diagram = match parse_yaml_file(input) {
        Ok(d) => d,
        Err(e) => return handle_parse_error(e),
    };

    let theme = resolve_theme(theme_name);

    let plan = match compute_layout(&diagram) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: layout failed: {e}");
            return ExitCode::from(2);
        }
    };

    // PNG produces binary output, handle separately.
    if format == "png" {
        let svg = match render_svg(&plan, &theme, width) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: SVG render failed: {e}");
                return ExitCode::from(2);
            }
        };
        let png_bytes = match render_png(&svg, scale) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("error: PNG render failed: {e}");
                return ExitCode::from(2);
            }
        };
        let output_path = match output {
            Some(p) => p.to_path_buf(),
            None => input.with_extension("png"),
        };
        return match std::fs::write(&output_path, &png_bytes) {
            Ok(()) => {
                eprintln!("wrote {}", output_path.display());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: failed to write output: {e}");
                ExitCode::from(2)
            }
        };
    }

    let (rendered, ext) = match format {
        "html" => match render_html(&plan, &theme) {
            Ok(h) => (h, "html"),
            Err(e) => {
                eprintln!("error: render failed: {e}");
                return ExitCode::from(2);
            }
        },
        "svg" => match render_svg(&plan, &theme, width) {
            Ok(s) => (s, "svg"),
            Err(e) => {
                eprintln!("error: render failed: {e}");
                return ExitCode::from(2);
            }
        },
        _ => {
            eprintln!("error: unsupported format '{format}' (use html, svg, or png)");
            return ExitCode::from(2);
        }
    };

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => input.with_extension(ext),
    };

    match std::fs::write(&output_path, &rendered) {
        Ok(()) => {
            eprintln!("wrote {}", output_path.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: failed to write output: {e}");
            ExitCode::from(2)
        }
    }
}

fn cmd_validate(input: &PathBuf) -> ExitCode {
    match parse_yaml_file(input) {
        Ok(diagram) => {
            let layer_count = diagram.layers().len();
            let edge_count = diagram.edges().len();
            let legend_count = diagram.legend().len();
            eprintln!(
                "valid: {layer_count} layers, {edge_count} edges, {legend_count} legend entries"
            );
            ExitCode::SUCCESS
        }
        Err(e) => handle_parse_error(e),
    }
}

fn cmd_init() -> ExitCode {
    print!(
        r#"diagram:
  title:
    text: "system architecture"
    accent: "myproject"
  subtitle: "a brief description of the system"
  theme: dark

layers:
  - tier:
      id: clients
      label: "Clients"
      nodes:
        - id: web-app
          kind: system
          color: blue
          icon: "◇"
          title: "Web App"
          description: "The main user-facing application"
          tech:
            - "React"
            - "TypeScript"

  - connector:
      style: line
      label: "HTTPS"

  - tier:
      id: services
      label: "Services"
      layout:
        grid:
          columns: 2
      nodes:
        - id: api
          kind: component
          color: green
          icon: "◈"
          title: "API Server"
          description: "REST API gateway"
          tech:
            - "Rust"
            - "axum"
        - id: worker
          kind: component
          color: amber
          icon: "◈"
          title: "Worker"
          description: "Background job processor"
          tech:
            - "tokio"

legend:
  - color: blue
    label: "Clients"
  - color: green
    label: "API"
  - color: amber
    label: "Workers"

edges:
  - from: web-app
    to: api
    kind: uses
    label: "REST calls"
  - from: api
    to: worker
    kind: uses
    label: "job dispatch"
"#
    );
    ExitCode::SUCCESS
}

fn cmd_themes() -> ExitCode {
    eprintln!("Available themes:");
    eprintln!("  dark  — Taproot dark theme (default)");
    ExitCode::SUCCESS
}

fn resolve_theme(name: &str) -> Theme {
    match name {
        "dark" => Theme::dark(),
        path => {
            // Try loading as a file path
            match std::fs::read_to_string(path) {
                Ok(content) => match serde_yml::from_str(&content) {
                    Ok(theme) => theme,
                    Err(e) => {
                        eprintln!("warning: failed to parse theme '{path}': {e}");
                        eprintln!("falling back to dark theme");
                        Theme::dark()
                    }
                },
                Err(_) => {
                    eprintln!("warning: unknown theme '{name}', using dark");
                    Theme::dark()
                }
            }
        }
    }
}

fn handle_parse_error(err: ParseError) -> ExitCode {
    eprintln!("error: {err}");
    match err {
        ParseError::Yaml(_) | ParseError::Json(_) | ParseError::Io(_) => ExitCode::from(2),
        ParseError::Validation(_) => ExitCode::from(1),
        _ => ExitCode::from(2),
    }
}
