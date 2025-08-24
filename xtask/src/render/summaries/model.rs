//! summary: Summaries表で使うデータ型（Section, Row）の定義
//! path: xtask/src/render/summaries/model.rs

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Section {
    Game,
    PlatformDesktop,
    PlatformWeb,
    Render,
    Xtask,
}

impl Section {
    pub fn title(&self) -> &'static str {
        match self {
            Section::Game => "### 🕹 game",
            Section::PlatformDesktop => "### 💻 platform/desktop",
            Section::PlatformWeb => "### 🌐 platform/web",
            Section::Render => "### 🎨 render",
            Section::Xtask => "### 🛠 xtask",
        }
    }

    /// 出力順を固定
    pub fn order() -> &'static [Section] {
        &[
            Section::Game,
            Section::PlatformDesktop,
            Section::PlatformWeb,
            Section::Render,
            Section::Xtask,
        ]
    }
}

#[derive(Debug)]
pub struct Row {
    pub rel_path: String,
    pub lines: usize,
    pub status: &'static str,
    pub summary: Option<String>,
}
