//! summary: integration_min (desktop) — AppBuilder で起動
//! path: examples/desktop/integration_min/src/main.rs

use anyhow::Result;

fn main() -> Result<()> {
    engine::AppBuilder::new()
        .clear_color([0.07, 0.10, 0.18, 1.0])
        .initial_size(1280, 720)
        .prefer_high_performance(true)
        .run()
}
