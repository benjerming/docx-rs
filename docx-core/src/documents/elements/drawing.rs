use super::*;
use serde::{ser::*, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Drawing {
    #[serde(flatten)]
    pub data: Option<DrawingData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawingData {
    Pic(Pic),
    TextBox(TextBox),
}

impl Serialize for DrawingData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DrawingData::Pic(ref pic) => {
                let mut t = serializer.serialize_struct("Pic", 2)?;
                t.serialize_field("type", "pic")?;
                t.serialize_field("data", pic)?;
                t.end()
            }
            DrawingData::TextBox(ref text_box) => {
                let mut t = serializer.serialize_struct("TextBox", 2)?;
                t.serialize_field("type", "textBox")?;
                t.serialize_field("data", text_box)?;
                t.end()
            }
        }
    }
}

impl Drawing {
    pub fn new() -> Drawing {
        Default::default()
    }

    pub fn pic(mut self, pic: Pic) -> Drawing {
        self.data = Some(DrawingData::Pic(pic));
        self
    }

    pub fn text_box(mut self, t: TextBox) -> Drawing {
        self.data = Some(DrawingData::TextBox(t));
        self
    }
}

impl BuildXML for Drawing {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let b = XMLBuilder::from(stream);
        let mut b = b.open_drawing()?;

        match &self.data {
            Some(DrawingData::Pic(p)) => {
                if let DrawingPositionType::Inline = p.position_type {
                    b = b.open_wp_inline(
                        &format!("{}", p.dist_t),
                        &format!("{}", p.dist_b),
                        &format!("{}", p.dist_l),
                        &format!("{}", p.dist_r),
                    )?
                } else {
                    b = b
                        .open_wp_anchor(
                            &format!("{}", p.dist_t),
                            &format!("{}", p.dist_b),
                            &format!("{}", p.dist_l),
                            &format!("{}", p.dist_r),
                            "0",
                            if p.simple_pos { "1" } else { "0" },
                            "0",
                            "0",
                            if p.layout_in_cell { "1" } else { "0" },
                            &format!("{}", p.relative_height),
                        )?
                        .simple_pos(
                            &format!("{}", p.simple_pos_x),
                            &format!("{}", p.simple_pos_y),
                        )?
                        .open_position_h(&format!("{}", p.relative_from_h))?;

                    match p.position_h {
                        DrawingPosition::Offset(x) => {
                            let x = format!("{}", x as u32);
                            b = b.pos_offset(&x)?.close()?;
                        }
                        DrawingPosition::Align(x) => {
                            b = b.align(&x.to_string())?.close()?;
                        }
                    }

                    b = b.open_position_v(&format!("{}", p.relative_from_v))?;

                    match p.position_v {
                        DrawingPosition::Offset(y) => {
                            let y = format!("{}", y as u32);
                            b = b.pos_offset(&y)?.close()?;
                        }
                        DrawingPosition::Align(a) => {
                            b = b.align(&a.to_string())?.close()?;
                        }
                    }
                }

                let w = format!("{}", p.size.0);
                let h = format!("{}", p.size.1);
                b = b
                    // Please see 20.4.2.7 extent (Drawing Object Size)
                    // One inch equates to 914400 EMUs and a centimeter is 360000
                    .wp_extent(&w, &h)?
                    .wp_effect_extent("0", "0", "0", "0")?;
                if p.allow_overlap {
                    b = b.wrap_none()?;
                } else if p.position_type == DrawingPositionType::Anchor {
                    b = b.wrap_square("bothSides")?;
                }
                b = b
                    .wp_doc_pr("1", "Figure")?
                    .open_wp_c_nv_graphic_frame_pr()?
                    .a_graphic_frame_locks(
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "1",
                    )?
                    .close()?
                    .open_a_graphic("http://schemas.openxmlformats.org/drawingml/2006/main")?
                    .open_a_graphic_data(
                        "http://schemas.openxmlformats.org/drawingml/2006/picture",
                    )?
                    .add_child(&p.clone())?
                    .close()?
                    .close()?;
            }
            Some(DrawingData::TextBox(t)) => {
                if let DrawingPositionType::Inline = t.position_type {
                    b = b.open_wp_inline(
                        &format!("{}", t.dist_t),
                        &format!("{}", t.dist_b),
                        &format!("{}", t.dist_l),
                        &format!("{}", t.dist_r),
                    )?
                } else {
                    b = b
                        .open_wp_anchor(
                            &format!("{}", t.dist_t),
                            &format!("{}", t.dist_b),
                            &format!("{}", t.dist_l),
                            &format!("{}", t.dist_r),
                            "0",
                            if t.simple_pos { "1" } else { "0" },
                            "0",
                            "0",
                            if t.layout_in_cell { "1" } else { "0" },
                            &format!("{}", t.relative_height),
                        )?
                        .simple_pos(
                            &format!("{}", t.simple_pos_x),
                            &format!("{}", t.simple_pos_y),
                        )?
                        .open_position_h(&format!("{}", t.relative_from_h))?;

                    match t.position_h {
                        DrawingPosition::Offset(x) => {
                            let x = format!("{}", x as u32);
                            b = b.pos_offset(&x)?.close()?;
                        }
                        DrawingPosition::Align(x) => {
                            b = b.align(&x.to_string())?.close()?;
                        }
                    }

                    b = b.open_position_v(&format!("{}", t.relative_from_v))?;

                    match t.position_v {
                        DrawingPosition::Offset(y) => {
                            let y = format!("{}", y as u32);
                            b = b.pos_offset(&y)?.close()?;
                        }
                        DrawingPosition::Align(a) => {
                            b = b.align(&a.to_string())?.close()?;
                        }
                    }
                }

                let w = format!("{}", t.size.0);
                let h = format!("{}", t.size.1);
                b = b.wp_extent(&w, &h)?.wp_effect_extent("0", "0", "0", "0")?;
                if t.allow_overlap {
                    b = b.wrap_none()?;
                } else if t.position_type == DrawingPositionType::Anchor {
                    b = b.wrap_square("bothSides")?;
                }
                b = b
                    .wp_doc_pr("1", &t.name)?
                    .open_wp_c_nv_graphic_frame_pr()?
                    .close()?
                    .open_a_graphic("http://schemas.openxmlformats.org/drawingml/2006/main")?
                    .open_a_graphic_data(
                        "http://schemas.microsoft.com/office/word/2010/wordprocessingShape",
                    )?
                    .open_wp_shape()?
                    .wps_cnv_sp_pr_tx_box("1")?
                    .open_wps_sp_pr("auto")?
                    .open_a_xfrm()?
                    .a_off("0", "0")?
                    .a_ext(&w, &h)?
                    .close()?
                    .open_a_prst_geom("rect")?
                    .a_av_lst()?
                    .close()?;

                match &t.style.fill {
                    Some(TextBoxFill::Solid { color }) => {
                        b = b.open_a_solid_fill()?.a_srgb_color(color)?.close()?;
                    }
                    Some(TextBoxFill::NoFill) => {
                        b = b.a_no_fill()?;
                    }
                    None => {}
                }
                match &t.style.line {
                    Some(TextBoxLine::Solid { color, width_emu }) => {
                        b = b.open_a_ln_with_w(&format!("{}", width_emu))?
                            .open_a_solid_fill()?
                            .a_srgb_color(color)?
                            .close()?
                            .close()?;
                    }
                    Some(TextBoxLine::NoLine) => {
                        b = b.open_a_ln()?.a_no_fill()?.close()?;
                    }
                    None => {}
                }

                b = b
                    .close()? // wps:spPr
                    .open_wp_text_box()?
                    .open_text_box_content()?
                    .add_children(&t.children)?
                    .close()? // w:txbxContent
                    .close()? // wps:txbx
                    .wps_body_pr()?
                    .close()? // wps:wsp
                    .close()? // a:graphicData
                    .close()?; // a:graphic
            }
            None => {
                unimplemented!()
            }
        }
        b.close()?.close()?.into_inner()
    }
}

#[cfg(test)]
mod tests {

    use crate::xml::test_utils::assert_xml_eq;

    use super::*;
    use std::str;

    #[test]
    fn test_drawing_build_with_pic() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240);
        let d = Drawing::new().pic(pic).build();
        assert_xml_eq(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_pic_overlap() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240).overlapping();
        let d = Drawing::new().pic(pic).build();
        assert_xml_eq(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapNone /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_pic_align_right() {
        let mut pic = Pic::new_with_dimensions(Vec::new(), 320, 240).floating();
        pic = pic.relative_from_h(RelativeFromHType::Column);
        pic = pic.relative_from_v(RelativeFromVType::Paragraph);
        pic = pic.position_h(DrawingPosition::Align(PicAlign::Right));
        let d = Drawing::new().pic(pic).build();
        assert_xml_eq(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" allowOverlap="0" behindDoc="0" locked="0" layoutInCell="0" relativeHeight="190500"><wp:simplePos x="0" y="0" /><wp:positionH relativeFrom="column"><wp:align>right</wp:align></wp:positionH><wp:positionV relativeFrom="paragraph"><wp:posOffset>0</wp:posOffset></wp:positionV><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapSquare wrapText="bothSides" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:anchor></w:drawing>"#
        );
    }

    #[test]
    fn test_issue686() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240)
            .size(320 * 9525, 240 * 9525)
            .floating()
            .offset_x(300 * 9525)
            .offset_y(400 * 9525);

        let d = Drawing::new().pic(pic).build();
        assert_xml_eq(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" allowOverlap="0" behindDoc="0" locked="0" layoutInCell="0" relativeHeight="190500"><wp:simplePos x="0" y="0" /><wp:positionH relativeFrom="margin"><wp:posOffset>2857500</wp:posOffset></wp:positionH><wp:positionV relativeFrom="margin"><wp:posOffset>3810000</wp:posOffset></wp:positionV><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapSquare wrapText="bothSides" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:anchor></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_floating_text_box() {
        let tb = TextBox::new()
            .size(914400, 914400)
            .floating()
            .overlapping()
            .relative_from_h(RelativeFromHType::Page)
            .relative_from_v(RelativeFromVType::Page)
            .offset_x(360000)
            .offset_y(720000)
            .name("Demo Box")
            .style(
                TextBoxStyle::new()
                    .solid_fill("FFF2CC")
                    .solid_line("CC0000", 12700),
            )
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello box")));

        let d = Drawing::new().text_box(tb).build();
        assert_xml_eq(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" allowOverlap="0" behindDoc="0" locked="0" layoutInCell="0" relativeHeight="190500"><wp:simplePos x="0" y="0" /><wp:positionH relativeFrom="page"><wp:posOffset>360000</wp:posOffset></wp:positionH><wp:positionV relativeFrom="page"><wp:posOffset>720000</wp:posOffset></wp:positionV><wp:extent cx="914400" cy="914400" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapNone /><wp:docPr id="1" name="Demo Box" /><wp:cNvGraphicFramePr /><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr txBox="1" /><wps:spPr bwMode="auto"><a:xfrm><a:off x="0" y="0" /><a:ext cx="914400" cy="914400" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom><a:solidFill><a:srgbClr val="FFF2CC" /></a:solidFill><a:ln w="12700"><a:solidFill><a:srgbClr val="CC0000" /></a:solidFill></a:ln></wps:spPr><wps:txbx><w:txbxContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello box</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr rot="0" vert="horz" wrap="square" lIns="91440" tIns="45720" rIns="91440" bIns="45720" anchor="t" anchorCtr="0" compatLnSpc="1" /></wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing>"#
        );
    }
}
