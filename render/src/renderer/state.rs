use winit::dpi::PhysicalSize;

#[derive(Clone)]
pub struct RenderState {
    pub clear_color: wgpu::Color,
    pub size: PhysicalSize<u32>,
    pub sample_count: u32,
}

impl RenderState {
    pub fn new(size: PhysicalSize<u32>, msaa4_feature_enabled: bool) -> Self {
        let sample_count = if msaa4_feature_enabled { 4 } else { 1 };
        Self {
            clear_color: wgpu::Color { r: 0.06, g: 0.07, b: 0.09, a: 1.0 },
            size,
            sample_count,
        }
    }
    pub fn set_clear_color(&mut self, rgba: [f32; 4]) {
        self.clear_color = wgpu::Color {
            r: rgba[0] as f64, g: rgba[1] as f64, b: rgba[2] as f64, a: rgba[3] as f64
        };
    }
}
