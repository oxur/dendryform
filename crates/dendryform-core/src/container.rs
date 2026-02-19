//! Container type for nested sub-diagrams.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::color::Color;
use crate::layer::Layer;

/// The border style of a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ContainerBorder {
    /// Rounded solid border, opaque background.
    Solid,
    /// Dashed border, tinted background matching label color.
    Dashed,
}

impl fmt::Display for ContainerBorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solid => f.write_str("solid"),
            Self::Dashed => f.write_str("dashed"),
        }
    }
}

/// A bordered box with a floating label that contains nested layers.
///
/// Used for server boundaries, subsystem groupings, etc. Containers
/// nest recursively — a tier inside a container can itself have a container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Container {
    label: String,
    border: ContainerBorder,
    label_color: Color,
    layers: Vec<Layer>,
}

impl Container {
    /// Creates a new container.
    pub fn new(
        label: &str,
        border: ContainerBorder,
        label_color: Color,
        layers: Vec<Layer>,
    ) -> Self {
        Self {
            label: label.to_owned(),
            border,
            label_color,
            layers,
        }
    }

    /// Returns the floating label text.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the border style.
    pub fn border(&self) -> ContainerBorder {
        self.border
    }

    /// Returns the label color.
    pub fn label_color(&self) -> Color {
        self.label_color
    }

    /// Returns the nested layers.
    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_display() {
        assert_eq!(format!("{}", ContainerBorder::Solid), "solid");
        assert_eq!(format!("{}", ContainerBorder::Dashed), "dashed");
    }

    #[test]
    fn test_serde_border() {
        let border = ContainerBorder::Dashed;
        let json = serde_json::to_string(&border).unwrap();
        assert_eq!(json, "\"dashed\"");
    }
}
