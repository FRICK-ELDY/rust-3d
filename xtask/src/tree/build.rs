//! summary: 走査結果からツリー構築
//! path: xtask/src/tree/build.rs

use camino::Utf8PathBuf;
use std::collections::BTreeMap;

use super::{model::Node, scan::scan_paths};
use crate::{
    config::Config,
    summary::{is_rs_or_wgsl, read_first_line_summary},
};

pub fn build_tree(cfg: &Config) -> anyhow::Result<Node> {
    let files = scan_paths(cfg);

    // ノード化
    let mut nodes: BTreeMap<Utf8PathBuf, Node> = BTreeMap::new();
    for p in &files {
        let up = Utf8PathBuf::from_path_buf(p.clone()).unwrap();
        let is_dir = p.is_dir();
        let summary = if !is_dir && is_rs_or_wgsl(&up) {
            read_first_line_summary(up.as_std_path()).ok().flatten()
        } else {
            None
        };
        nodes.insert(
            up.clone(),
            Node {
                path: up,
                is_dir,
                children: vec![],
                summary,
            },
        );
    }

    // ルート
    let mut root = Node {
        path: cfg.repo_root.clone(),
        is_dir: true,
        children: vec![],
        summary: None,
    };

    // 親→子リスト
    let mut parent_to_children: BTreeMap<Utf8PathBuf, Vec<Utf8PathBuf>> = BTreeMap::new();
    for key in nodes.keys() {
        let parent = key
            .parent()
            .map(Utf8PathBuf::from)
            .unwrap_or_else(|| cfg.repo_root.clone());
        parent_to_children
            .entry(parent)
            .or_default()
            .push(key.clone());
    }

    // 再帰構築
    fn build_subtree(
        parent: &Utf8PathBuf,
        nodes: &BTreeMap<Utf8PathBuf, Node>,
        parent_to_children: &BTreeMap<Utf8PathBuf, Vec<Utf8PathBuf>>,
        cfg_root: &Utf8PathBuf,
    ) -> Vec<Node> {
        let mut out = vec![];
        if let Some(children) = parent_to_children.get(parent) {
            for child_path in children {
                if let Some(n) = nodes.get(child_path) {
                    if n.is_dir {
                        out.push(Node {
                            path: n.path.clone(),
                            is_dir: true,
                            children: build_subtree(
                                child_path,
                                nodes,
                                parent_to_children,
                                cfg_root,
                            ),
                            summary: None,
                        });
                    } else {
                        out.push(Node {
                            path: n.path.clone(),
                            is_dir: false,
                            children: vec![],
                            summary: n.summary.clone(),
                        });
                    }
                }
            }
        }
        out
    }

    root.children = build_subtree(&cfg.repo_root, &nodes, &parent_to_children, &cfg.repo_root);
    Ok(root)
}
