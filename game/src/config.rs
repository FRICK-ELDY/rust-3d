use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub window_width:  u32,
    pub window_height: u32,
    pub fullscreen:    bool,
    pub vsync:         bool,
    /// クリアカラー RGBA（0.0..1.0）
    pub clear_color:   [f32; 4],
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 1280,
            window_height: 720,
            fullscreen: false,
            vsync: true,
            clear_color: [0.02, 0.07, 0.12, 1.0],
        }
    }
}

impl GameConfig {
    pub fn from_toml_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str::<GameConfig>(s)
    }
}
