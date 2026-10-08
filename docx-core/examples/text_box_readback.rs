//! 验证 examples/text_box.rs 生成的浮动文本框能被 reader 正确读回。
//!
//! 运行前先生成文件: cargo run --example text_box
//! 然后:         cargo run --example text_box_readback

use docx_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("./text_box.docx")?;
    let docx = read_docx_with_options(&bytes, ReadDocxOptions::default().with_image_previews(false))?;

    let json = docx.json();
    let boxes: Vec<&str> = json.matches("\"textBox\"").collect();
    eprintln!("[info] 读回成功,JSON 长度 {},textBox 节点数 {}", json.len(), boxes.len());

    // 关键断言:两个浮动框的内容与样式往返无损
    for (label, needle) in [
        ("框1名称", "项目速览"),
        ("框1填充色", "FFF8E1"),
        ("框1边框色", "D2691E"),
        ("框1表格文本", "里程碑"),
        ("框2名称", "签注区"),
        ("框2边框色", "2E74B5"),
    ] {
        let hit = json.contains(needle);
        eprintln!("[{}] {} ({})", if hit { "info" } else { "error" }, label, needle);
        if !hit {
            std::process::exit(1);
        }
    }
    eprintln!("[info] 浮动文本框写入->读回 往返验证通过");
    Ok(())
}
