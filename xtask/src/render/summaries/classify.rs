//! summary: パスから Section を分類するユーティリティ
//! path: xtask/src/render/summaries/classify.rs

use super::model::Section;

/// relative path の先頭から Section を分類
pub fn classify_section(rel: &str) -> Option<Section> {
    if rel.starts_with("game/") {
        Some(Section::Game)
    } else if rel.starts_with("platform/desktop/") {
        Some(Section::PlatformDesktop)
    } else if rel.starts_with("platform/web/") {
        Some(Section::PlatformWeb)
    } else if rel.starts_with("render/") {
        Some(Section::Render)
    } else if rel.starts_with("xtask/") {
        Some(Section::Xtask)
    } else {
        None
    }
}
