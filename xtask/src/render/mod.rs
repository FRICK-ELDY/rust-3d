//! summary: WorkspaceLayout.md の Markdown 出力 (facade)
//! path: xtask/src/render/mod.rs

mod header;
mod summaries;
mod tree;
mod utils;

use crate::config::Config;
use crate::tree::Node;
use anyhow::Result;

pub fn render_markdown(cfg: &Config, root: &Node) -> Result<String> {
    let mut out = String::new();

    // 先頭メタ情報＆Legend
    header::write_header(&mut out, cfg);

    // Summaries（Path / Lines / Status / Summary）
    summaries::write_summaries(&mut out, cfg, root)?;

    // Tree 表示（サマリの後）
    tree::write_tree(&mut out, root, cfg.truncate);

    Ok(out)
}
