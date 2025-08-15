#[derive(Clone, Debug)]
pub struct RenderConfig {
    pub msaa_samples: u32,
}
impl Default for RenderConfig {
    fn default() -> Self { Self { msaa_samples: 1 } }
}
