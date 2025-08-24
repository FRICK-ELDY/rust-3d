//! summary: Tree 表示（コードブロック）
//! path: xtask/src/render/tree.rs

use crate::tree::Node;
use camino::Utf8Path;

pub fn write_tree(out: &mut String, root: &Node, truncate: usize) {
    out.push_str("## Directory / File Tree\n\n");
    out.push_str("```\n");
    out.push_str("root/\n");
    rec(out, root, "", truncate);
    out.push_str("```\n\n");
}

fn rec(out: &mut String, node: &Node, prefix: &str, truncate: usize) {
    let len = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let is_last = i + 1 == len;
        let (branch, next_prefix) = if is_last {
            ("└─ ", format!("{}   ", prefix))
        } else {
            ("├─ ", format!("{}│  ", prefix))
        };

        if child.is_dir {
            out.push_str(prefix);
            out.push_str(branch);
            out.push_str(&name_of(&child.path, truncate));
            out.push_str("/\n");
            rec(out, child, &next_prefix, truncate);
        } else {
            out.push_str(prefix);
            out.push_str(branch);
            out.push_str(&name_of(&child.path, truncate));
            if let Some(s) = &child.summary {
                out.push_str(" — ");
                out.push_str(s);
            }
            out.push('\n');
        }
    }
}

fn name_of(p: &Utf8Path, truncate: usize) -> String {
    let name = p.file_name().unwrap_or_else(|| p.as_str());
    if truncate > 0 && name.len() > truncate {
        format!("{}…", &name[..truncate])
    } else {
        name.to_string()
    }
}
