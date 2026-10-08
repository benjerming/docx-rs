use super::XMLBuilder;
use super::XmlEvent;

use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    open!(
        open_wp_inline,
        "wp:inline",
        "distT",
        "distB",
        "distL",
        "distR"
    );

    open!(
        open_wp_anchor,
        "wp:anchor",
        "distT",
        "distB",
        "distL",
        "distR",
        "simplePos",
        "allowOverlap",
        "behindDoc",
        "locked",
        "layoutInCell",
        "relativeHeight"
    );

    open!(open_a_graphic, "a:graphic", "xmlns:a");
    open!(open_a_graphic_data, "a:graphicData", "uri");
    closed!(wp_extent, "wp:extent", "cx", "cy");
    closed!(wp_effect_extent, "wp:effectExtent", "b", "l", "r", "t");
    closed!(wp_doc_pr, "wp:docPr", "id", "name");
    open!(open_wp_c_nv_graphic_frame_pr, "wp:cNvGraphicFramePr");
    closed!(
        a_graphic_frame_locks,
        "a:graphicFrameLocks",
        "xmlns:a",
        "noChangeAspect"
    );

    closed!(simple_pos, "wp:simplePos", "x", "y");
    open!(open_position_h, "wp:positionH", "relativeFrom");
    open!(open_position_v, "wp:positionV", "relativeFrom");
    closed_with_child!(pos_offset, "wp:posOffset");
    closed_with_child!(align, "wp:align");
    closed!(wrap_none, "wp:wrapNone");
    closed!(wrap_square, "wp:wrapSquare", "wrapText");

    closed!(wps_cnv_sp_pr_tx_box, "wps:cNvSpPr", "txBox");
    open!(open_wps_sp_pr, "wps:spPr", "bwMode");
    open!(open_a_ln, "a:ln");
    open!(open_a_ln_with_w, "a:ln", "w");
    closed!(a_no_fill, "a:noFill");
    open!(open_a_solid_fill, "a:solidFill");
    closed!(a_srgb_color, "a:srgbClr", "val");

    /// `<wps:bodyPr/>` with the defaults Word emits for a plain text box.
    /// Insets are EMU (Word defaults: 91440 left/right, 45720 top/bottom).
    pub(crate) fn wps_body_pr(self) -> crate::xml::writer::Result<Self> {
        self.write(
            XmlEvent::start_element("wps:bodyPr")
                .attr("rot", "0")
                .attr("vert", "horz")
                .attr("wrap", "square")
                .attr("lIns", "91440")
                .attr("tIns", "45720")
                .attr("rIns", "91440")
                .attr("bIns", "45720")
                .attr("anchor", "t")
                .attr("anchorCtr", "0")
                .attr("compatLnSpc", "1"),
        )?
        .close()
    }
}
