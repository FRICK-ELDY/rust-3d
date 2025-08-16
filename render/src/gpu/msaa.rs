use wgpu::{Device, Texture, TextureFormat, TextureView};

#[derive(Debug)]
pub struct MsaaTargets {
    pub color_tex: Texture,
    pub color_view: TextureView,
    pub sample_count: u32,
    pub format: TextureFormat,
}

impl MsaaTargets {
    pub fn new(
        device: &Device,
        width: u32,
        height: u32,
        format: TextureFormat,
        sample_count: u32,
    ) -> Self {
        let color_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render/msaa-color"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let color_view = color_tex.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            color_tex,
            color_view,
            sample_count,
            format,
        }
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        *self = Self::new(
            device,
            width.max(1),
            height.max(1),
            self.format,
            self.sample_count,
        );
    }
}

pub fn new_if_needed(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> Option<MsaaTargets> {
    (sample_count > 1).then(|| MsaaTargets::new(device, width, height, format, sample_count))
}
