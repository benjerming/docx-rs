//! 浮动文本框 demo:
//! - 两个文本框均以「页面」为参照定位(wp:anchor + relativeFrom="page")
//! - 框内可容纳任意段落与表格
//! - 框的颜色(填充 + 边框)与文字样式(字体/字号/颜色/加粗/斜体)均可自定义
//!
//! 运行: cargo run --example text_box   (在 docx-core 目录下生成 text_box.docx)

use docx_rs::*;

/// 1 cm = 360000 EMU,用于 anchor 偏移与文本框尺寸
const CM: u32 = 360_000;
/// 1 pt = 12700 EMU,用于边框线宽
const PT: u32 = 12_700;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("./text_box.docx");
    eprintln!("[info] 开始构建浮动文本框 demo 文档");

    // ---------------------------------------------------------------
    // 框 1:页面右上部,自定义填充色 + 边框色/线宽,文字环绕(wrapSquare)
    // ---------------------------------------------------------------
    let overview_table = Table::new(vec![
        TableRow::new(vec![
            styled_cell("指标", "FFFFFF", true),
            styled_cell("数值", "FFFFFF", true),
        ]),
        TableRow::new(vec![
            styled_cell("里程碑", "333333", false),
            styled_cell("2026 Q4", "333333", false),
        ]),
        TableRow::new(vec![
            styled_cell("覆盖率", "333333", false),
            styled_cell("87.5%", "C00000", false),
        ]),
    ])
    .set_grid(vec![2400, 2400])
    .set_borders(
        TableBorders::new()
            .set(TableBorder::new(TableBorderPosition::Top).border_type(BorderType::Single).size(4).color("1F4E79"))
            .set(TableBorder::new(TableBorderPosition::Bottom).border_type(BorderType::Single).size(4).color("1F4E79"))
            .set(TableBorder::new(TableBorderPosition::Left).border_type(BorderType::Single).size(4).color("1F4E79"))
            .set(TableBorder::new(TableBorderPosition::Right).border_type(BorderType::Single).size(4).color("1F4E79"))
            .set(TableBorder::new(TableBorderPosition::InsideH).border_type(BorderType::Single).size(4).color("8EAADB"))
            .set(TableBorder::new(TableBorderPosition::InsideV).border_type(BorderType::Single).size(4).color("8EAADB")),
    );

    let box1 = TextBox::new()
        .name("项目速览")
        // 相对「页面」定位:距左边 11cm、距顶端 4cm,尺寸 8.5cm x 8.2cm
        .floating()
        .relative_from_h(RelativeFromHType::Page)
        .relative_from_v(RelativeFromVType::Page)
        .offset_x(11 * CM as i32)
        .offset_y(4 * CM as i32)
        .size(CM * 17 / 2, CM * 8 + CM / 5)
        // 框的颜色:浅黄底 + 橙色 1.5pt 边框
        .style(
            TextBoxStyle::new()
                .solid_fill("FFF8E1")
                .solid_line("D2691E", PT * 3 / 2),
        )
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("项目速览")
                    .bold()
                    .size(30) // 半磅为单位:30 = 15pt
                    .color("1F4E79")
                    .fonts(yahei()),
            ),
        )
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("Floating text box anchored to the page")
                    .italic()
                    .underline("single")
                    .size(18) // 9pt
                    .color("808080")
                    .fonts(yahei()),
            ),
        )
        .add_table(overview_table)
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("状态:").bold().size(20).fonts(yahei()))
                .add_run(
                    Run::new()
                        .add_text("按期推进")
                        .size(20)
                        .highlight("FFFF00")
                        .color("C00000")
                        .fonts(yahei()),
                ),
        );

    // ---------------------------------------------------------------
    // 框 2:页面左下部,透明底(no fill)+ 细蓝边,不与正文交互(wrapNone)
    // ---------------------------------------------------------------
    let box2 = TextBox::new()
        .name("签注区")
        .floating()
        .overlapping() // allowOverlap -> 生成 <wp:wrapNone/>,正文直接穿过
        .relative_from_h(RelativeFromHType::Page)
        .relative_from_v(RelativeFromVType::Page)
        .offset_x(2 * CM as i32)
        .offset_y(20 * CM as i32)
        .size(CM * 7, CM * 4)
        .style(
            TextBoxStyle::new()
                .no_fill()
                .solid_line("2E74B5", PT * 3 / 4), // 0.75pt
        )
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_text("—— 签注区 ——")
                        .italic()
                        .color("2E74B5")
                        .size(24) // 12pt
                        .fonts(yahei()),
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_text("审核人 / 日期")
                        .color("595959")
                        .size(18)
                        .fonts(yahei()),
                ),
        );

    // ---------------------------------------------------------------
    // 正文:说明文字 + 把两个框挂在正文首段(run 级 drawing)
    // ---------------------------------------------------------------
    let intro = Paragraph::new()
        .add_run(Run::new().add_text("浮动文本框 Demo").bold().size(32).fonts(yahei()))
        // anchor 挂在 run 上,但定位基准是「页面」
        .add_run(Run::new().add_text_box(box1))
        .add_run(Run::new().add_text_box(box2));

    let body = vec![
        intro,
        para("本文档由 docx-rs 生成,演示两个相对页面浮动的文本框:右上的「项目速览」框采用浅黄填充与橙色边框,内部包含标题段落、副标题段落、一张带边框与表头底纹的表格,以及使用高亮样式的状态行。"),
        para("页面左下方的「签注区」框演示另一种组合:透明填充、0.75pt 细蓝边框、居中的斜体文字,并且设置为 wrapNone,不参与正文的文字环绕。"),
        para("正文的这些段落会自动绕开右侧的环绕文本框(wrapSquare bothSides),这是 Word 中图文混排的常见版式;两个框的定位基准均为页面左上角,偏移量以 EMU 精确控制(1cm = 360000 EMU)。"),
        para("文字样式方面,本 demo 覆盖了字体(Microsoft YaHei)、字号(半磅单位)、字色、加粗、斜体、下划线与高亮;框的样式则覆盖了纯色填充、无填充、纯色边框及线宽控制。"),
    ];

    let file = std::fs::File::create(path).map_err(|e| {
        eprintln!("[error] 创建输出文件失败: {path:?}, 原因: {e}");
        e
    })?;
    let mut docx = Docx::new();
    for p in body {
        docx = docx.add_paragraph(p);
    }
    docx.pack(file).map_err(|e| {
        eprintln!("[error] 写入 docx 失败: {e}");
        e
    })?;
    eprintln!("[info] 已生成: {}", path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display());
    Ok(())
}

fn yahei() -> RunFonts {
    RunFonts::new()
        .ascii("Microsoft YaHei")
        .hi_ansi("Microsoft YaHei")
        .east_asia("Microsoft YaHei")
}

fn para(text: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text).size(21).fonts(yahei()))
}

/// 表格单元格:文字样式 + 底纹
fn styled_cell(text: &str, color: &str, bold: bool) -> TableCell {
    let mut run = Run::new().add_text(text).color(color).size(20).fonts(yahei());
    if bold {
        run = run.bold();
    }
    TableCell::new()
        .add_paragraph(Paragraph::new().add_run(run))
        .vertical_align(VAlignType::Center)
        .shading(Shading::new().fill(if bold { "1F4E79" } else { "FFFFFF" }))
}
