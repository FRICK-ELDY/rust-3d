//! summary: 先頭メタ情報とLegend出力
//! path: xtask/src/render/header.rs

use crate::config::Config;
use chrono::Local;

pub fn write_header(out: &mut String, cfg: &Config) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    out.push_str("# Workspace Layout\n\n");
    out.push_str(&format!("- Generated: {}\n", now));
    out.push_str(&format!("- Root: `{}`\n", cfg.repo_root));
    out.push_str(&format!(
        "- Max Depth: {}\n",
        cfg.max_depth.map(|d| d.to_string()).unwrap_or_else(|| "none".into())
    ));
    out.push_str(&format!(
        "- Excludes: `{}`\n\n",
        cfg.excludes.iter().cloned().collect::<Vec<_>>().join(", ")
    ));

    out.push_str("### Legend\n");
    out.push_str("- 0–4: ⚪ (無評価)\n");
    out.push_str("- 5–50: 🟢 (OK, 保持)\n");
    out.push_str("- 51–100: 🟡 (様子見, 早めに分割候補)\n");
    out.push_str("- 101–200: 🟠 (分割推奨)\n");
    out.push_str("- 200–: 🔴 (最優先で分割)\n\n");
}
