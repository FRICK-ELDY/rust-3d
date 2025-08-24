//! summary: ツリーノードのデータ構造
//! path: xtask/src/tree/model.rs

use camino::Utf8PathBuf;

#[derive(Debug)]
pub struct Node {
    pub path: Utf8PathBuf,
    pub is_dir: bool,
    pub children: Vec<Node>,
    pub summary: Option<String>, // .rs / .wgsl の 1 行目 summary
}
