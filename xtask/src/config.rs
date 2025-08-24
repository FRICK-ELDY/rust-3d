//! summary: 設定と既定の除外セット
//! path: xtask/src/config.rs

use camino::Utf8PathBuf;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Config {
    pub repo_root: Utf8PathBuf,
    pub excludes: BTreeSet<String>,
    pub max_depth: Option<usize>,
    pub truncate: usize,
    pub respect_gitignore: bool,
}

pub fn resolve_repo_root(root: Option<String>) -> anyhow::Result<Utf8PathBuf> {
    if let Some(r) = root {
        return Ok(Utf8PathBuf::from(r));
    }
    let here = Utf8PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let Some(parent) = here.parent() else {
        anyhow::bail!("Could not resolve repository root");
    };
    Ok(parent.to_owned())
}

pub fn default_excludes() -> BTreeSet<String> {
    [
        ".git",
        ".gitignore",
        "README.md",
        "WorkspaceLayout.md",
        "Cargo.lock",
        "target",
        "node_modules",
        "dist",
        "build",
        ".idea",
        ".vscode",
        ".github",
        ".dart_tool",
        "out",
        "bin",
        "docs",
        "assets",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn set_from_vec(v: &[String]) -> BTreeSet<String> {
    v.iter().cloned().collect()
}
