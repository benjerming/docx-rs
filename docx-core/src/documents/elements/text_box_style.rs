use serde::Serialize;

/// Fill of a text box shape, written into `wps:spPr`.
///
/// `None` in [`TextBoxStyle`](super::TextBoxStyle) leaves the element out so the
/// shape inherits the renderer default (Word draws a theme-dependent fill).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TextBoxFill {
    /// `<a:solidFill><a:srgbClr val="..."/></a:solidFill>` with an RRGGBB hex color.
    Solid { color: String },
    /// `<a:noFill/>` — transparent background.
    NoFill,
}

/// Outline of a text box shape, written into `wps:spPr`.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TextBoxLine {
    /// `<a:ln w="...">` with a solid RRGGBB stroke. Width is in EMU (12700 EMU = 1 pt).
    Solid { color: String, width_emu: u32 },
    /// `<a:ln><a:noFill/></a:ln>` — no outline.
    NoLine,
}

/// Shape-level styling (fill + outline) of a [`TextBox`](super::TextBox).
///
/// Both slots default to `None`, meaning "emit nothing and let the renderer
/// decide". Set them explicitly for reproducible output across Word and
/// LibreOffice.
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextBoxStyle {
    pub fill: Option<TextBoxFill>,
    pub line: Option<TextBoxLine>,
}

impl TextBoxStyle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn solid_fill(mut self, color: impl Into<String>) -> Self {
        self.fill = Some(TextBoxFill::Solid {
            color: color.into(),
        });
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill = Some(TextBoxFill::NoFill);
        self
    }

    /// Solid outline. `width_emu` is in EMU; 12700 EMU = 1 pt.
    pub fn solid_line(mut self, color: impl Into<String>, width_emu: u32) -> Self {
        self.line = Some(TextBoxLine::Solid {
            color: color.into(),
            width_emu,
        });
        self
    }

    pub fn no_line(mut self) -> Self {
        self.line = Some(TextBoxLine::NoLine);
        self
    }
}
