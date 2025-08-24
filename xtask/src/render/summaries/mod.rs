//! summary: Summaries表の生成のエントリポイント（セクション収集＋Markdown出力）
//! path: xtask/src/render/summaries/mod.rs

mod model;
mod classify;
mod collector;
mod writer;

use crate::config::Config;
use crate::tree::Node;
use anyhow::Result;
use model::{Section, Row};
use std::collections::BTreeMap;

/// 外部公開API：WorkspaceLayout.md の Summaries を書き込む
pub fn write_summaries(out: &mut String, cfg: &Config, root: &Node) -> Result<()> {
    out.push_str("## Summaries\n\n");

    // 1) セクション別に収集
    let mut sections: BTreeMap<Section, Vec<Row>> = BTreeMap::new();
    collector::collect_sectioned_rows(root, &mut sections, &cfg.repo_root);

    // 2) 出力（決め打ち順）
    writer::write_sections(out, sections);

    out.push('\n');
    Ok(())
}
