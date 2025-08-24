//! summary: Nodeツリーを走査してファイル行数・summaryを収集する処理
//! path: xtask/src/render/summaries/collector.rs

use super::classify::classify_section;
use super::model::{Row, Section};
use crate::tree::Node;
use camino::Utf8Path;
use std::collections::BTreeMap;
use std::fs;

use crate::render::utils::to_unix_sep;

/// 木を走査してセクション別に Row を収集
pub fn collect_sectioned_rows(
    node: &Node,
    sections: &mut BTreeMap<Section, Vec<Row>>,
    repo_root: &Utf8Path,
) {
    if !node.is_dir {
        if node
            .path
            .extension()
            .map(|e| e == "rs" || e == "wgsl")
            .unwrap_or(false)
        {
            let rel = node
                .path
                .strip_prefix(repo_root)
                .unwrap_or(node.path.as_path())
                .to_string();
            let rel = to_unix_sep(&rel);

            if let Some(sec) = classify_section(&rel) {
                let lines = count_lines(&node.path);
                let status = status_for(lines);
                let row = Row {
                    rel_path: rel,
                    lines,
                    status,
                    summary: node.summary.clone(),
                };
                sections.entry(sec).or_default().push(row);
            }
        }
    }
    for child in &node.children {
        collect_sectioned_rows(child, sections, repo_root);
    }
}

/// 行数カウント
pub fn count_lines(abs: &Utf8Path) -> usize {
    fs::read_to_string(abs).map(|s| s.lines().count()).unwrap_or(0)
}

/// 行数→ステータス（色付き絵文字）
pub fn status_for(lines: usize) -> &'static str {
    match lines {
        0..=4 => "⚪",
        5..=50 => "🟢",
        51..=100 => "🟡",
        101..=200 => "🟠",
        _ => "🔴",
    }
}
