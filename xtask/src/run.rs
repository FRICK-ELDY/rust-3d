//! summary: layout サブコマンドの実装
//! path: xtask/src/run.rs

use std::fs;

use crate::{
    config::{Config, default_excludes, resolve_repo_root, set_from_vec},
    render::render_markdown,
    tree::build::build_tree,
};

pub fn run_layout(
    root: Option<String>,
    exclude: Vec<String>,
    max_depth: Option<usize>,
    truncate: usize,
    respect_gitignore: bool,
) -> anyhow::Result<()> {
    let repo_root = resolve_repo_root(root)?;
    let config = Config {
        repo_root: repo_root.clone(),
        excludes: default_excludes()
            .union(&set_from_vec(&exclude))
            .cloned()
            .collect(),
        max_depth,
        truncate,
        respect_gitignore,
    };

    let tree = build_tree(&config)?;
    let md = render_markdown(&config, &tree)?;
    let out = repo_root.join("WorkspaceLayout.md");
    fs::write(&out, md)?;
    println!("✅ Generated: {}", out);
    Ok(())
}
