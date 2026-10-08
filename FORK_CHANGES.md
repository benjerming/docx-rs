# Fork changes (gaugo123/docx-rs)

This fork adds features MDtoWord needs, pending upstream adoption. Structured
for trivial rebasing onto new `bokuweb/docx-rs` releases.

**Upstream base:** track the tag/commit last rebased onto (update on each rebase).

## Upgrade procedure
1. `git fetch upstream && git rebase upstream/main`
2. Conflicts can only occur in the **Category-B** files below. Re-apply per notes.
3. `cargo test -p docx-rs -- --test-threads=1` ; `cargo build -p docx-rs`
4. `git push --force-with-lease origin feat/theme-color` (refreshes PR #895).
5. Rebuild MDtoWord: `cargo update -p docx-rs && cargo make ci`.

## Category A — isolated additions (NEW files, never conflict)
- `docx-core/src/documents/elements/style_ext.rs` — `Style::shading`
- `docx-core/src/documents/elements/paragraph_ext.rs` — `Paragraph::set_borders`
- `docx-core/src/types/theme_color.rs` — `ThemeColor` enum
- `docx-core/src/documents/elements/text_box_style.rs` — `TextBoxStyle`/`TextBoxFill`/`TextBoxLine` (shape fill + outline model for text boxes)
- `docx-core/examples/theme_color.rs`
- `docx-core/examples/text_box.rs` — floating text box demo (page-relative anchor, paragraphs + table inside, custom fill/border colors, run styling)
- `docx-core/examples/text_box_readback.rs` — write→read round-trip check for the demo
- one-line `mod` registrations in `documents/elements/mod.rs` and `types/mod.rs`

## Category B — edits to upstream-owned files (the only conflict surface)
- `documents/elements/color.rs` — 3 `Option<String>` theme fields + builders + `build_to` emission
- `xml_builder/elements.rs` — `color_with_theme` helper
- `documents/elements/{run_property,run,style}.rs` — theme delegators
- `reader/run_property.rs` — `read_color` by-attribute-name dispatch (also fixes a latent positional bug)
- `reader/run.rs` — reader round-trip test
- `documents/elements/drawing.rs` — implement the `DrawingData::TextBox` writer branch (was `unimplemented!`): full `wp:anchor`/`wp:inline` positioning, `wps:wsp` + `wps:spPr` (fill/outline) + `wps:txbx` + `wps:bodyPr`; plus a writer unit test
- `documents/elements/text_box.rs` — `style`/`name` fields, `add_paragraph`/`add_table`/`style`/`name`/`overlapping` builders
- `documents/elements/run.rs` — `pub fn add_text_box` (new); `add_drawing` promoted from `pub(crate)` to `pub`
- `xml_builder/drawing.rs` — `wps:cNvSpPr`/`wps:spPr`/`a:ln`/`a:noFill`/`a:solidFill`/`a:srgbClr`/`wps:bodyPr` emitters
- `reader/drawing.rs` — read back `wp:extent` (real size), `wp:docPr` name, and `wps:spPr` fill/outline into `TextBox` (`read_shape_style`); previously size stayed at the 100px default and style/name were dropped
- `reader/xml_element.rs` — `AXMLElement::{NoFill, SrgbClr}` variants
- `tests/snapshots/{lib,reader}__reader__read_textbox.snap` — fixture textbox now reports real `wp:extent` size
