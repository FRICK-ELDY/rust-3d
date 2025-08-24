//! summary: SectionごとのテーブルMarkdownを組み立てて出力する処理
//! path: xtask/src/render/summaries/writer.rs

use super::model::{Row, Section};
use crate::render::utils::{sanitize_md_cell, url_encode_path};
use std::collections::BTreeMap;

pub const BASE_URL: &str = "https://github.com/FRICK-ELDY/rust-3d/blob/main/";

/// 収集済みセクションを Markdown として出力
pub fn write_sections(out: &mut String, mut sections: BTreeMap<Section, Vec<Row>>) {
    let mut first = true;

    for sec in Section::order() {
        if !first {
            out.push_str("\n---\n\n");
        }
        first = false;

        out.push_str(sec.title());
        out.push('\n');

        out.push_str("| Path | Lines | Status | Summary |\n|------|------:|:------:|---------|\n");

        if let Some(rows) = sections.get_mut(sec) {
            rows.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

            if rows.is_empty() {
                out.push_str("| _no files_ | 0 | - | - |\n");
            } else {
                for r in rows.iter() {
                    let url = format!("{}{}", BASE_URL, url_encode_path(&r.rel_path));
                    let sum = r
                        .summary
                        .as_ref()
                        .map(|s| sanitize_md_cell(s))
                        .unwrap_or_else(|| "(no summary)".to_string());

                    out.push_str(&format!(
                        "| [{}]({}) | {} | {} | {} |\n",
                        r.rel_path, url, r.lines, r.status, sum
                    ));
                }
            }
        } else {
            out.push_str("| _no files_ | 0 | - | - |\n");
        }
    }
}
