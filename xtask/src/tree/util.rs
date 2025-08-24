//! summary: ツリー構築で使う小物ユーティリティ
//! path: xtask/src/tree/util.rs

use crate::config::Config;
use std::path::Path;

pub fn is_excluded_anywhere(p: &Path, cfg: &Config) -> bool {
    let Ok(rel) = p.strip_prefix(cfg.repo_root.as_std_path()) else {
        return false;
    };
    for comp in rel.components() {
        let s = comp.as_os_str().to_string_lossy();
        if cfg.excludes.contains(s.as_ref()) {
            return true;
        }
    }
    false
}

pub fn depth_from_root(p: &Path, root: &Path) -> usize {
    let Ok(rel) = p.strip_prefix(root) else {
        return 0;
    };
    rel.components().count()
}
