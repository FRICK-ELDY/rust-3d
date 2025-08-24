//! summary: ファイルシステム走査（ignoreを尊重）
//! path: xtask/src/tree/scan.rs

use ignore::WalkBuilder;
use std::path::PathBuf;

use crate::config::Config;

pub fn scan_paths(cfg: &Config) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let mut wb = WalkBuilder::new(cfg.repo_root.as_std_path());
    wb.follow_links(false)
        .hidden(false)
        .parents(true)
        .git_ignore(cfg.respect_gitignore)
        .git_exclude(cfg.respect_gitignore)
        .git_global(cfg.respect_gitignore);

    for dent in wb.build() {
        let Ok(d) = dent else { continue };
        let p = d.path();

        if p == cfg.repo_root.as_std_path() {
            continue;
        }
        if super::util::is_excluded_anywhere(p, cfg) {
            continue;
        }
        if let Some(maxd) = cfg.max_depth {
            if super::util::depth_from_root(p, cfg.repo_root.as_std_path()) > maxd {
                continue;
            }
        }
        files.push(p.to_path_buf());
    }

    // ディレクトリ優先、パス順
    files.sort_by(|a, b| {
        let ad = a.is_dir();
        let bd = b.is_dir();
        (!ad).cmp(&!bd).then(a.cmp(b))
    });

    files
}
