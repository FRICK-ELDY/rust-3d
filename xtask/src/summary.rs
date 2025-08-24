//! summary: .rs/.wgsl の1行目から summary を抽出
//! path: xtask/src/summary.rs

use anyhow::Context;
use camino::Utf8Path;
use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

pub fn is_rs_or_wgsl(p: &Utf8Path) -> bool {
    matches!(p.extension(), Some(ext) if ext == "rs" || ext == "wgsl")
}

pub fn read_first_line_summary(path: &Path) -> anyhow::Result<Option<String>> {
    let f = fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut reader = io::BufReader::new(f);
    let mut line = String::new();
    let _ = reader.read_line(&mut line)?;
    let line = line.trim_start_matches('\u{feff}').trim();

    // //! summary: ... / // summary: ... （前後空白緩め）
    let s = line
        .trim_start_matches('/')
        .trim_start_matches('/')
        .trim_start_matches('!')
        .trim();

    if let Some(rest) = s.strip_prefix("summary") {
        let rest = rest.trim_start_matches(':').trim();
        if !rest.is_empty() {
            return Ok(Some(rest.to_string()));
        }
    }
    Ok(None)
}
