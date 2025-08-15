use wgpu::{Device, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
           TextureView, TextureViewDescriptor, Extent3d, Sampler, SamplerDescriptor, CompareFunction,
           FilterMode, AddressMode};

pub struct DepthTarget {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub format: TextureFormat,
}

impl DepthTarget {
    pub const DEFAULT_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn create(device: &Device, width: u32, height: u32, format: TextureFormat) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("depth-texture"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("depth-sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });
        Self { texture, view, sampler, format }
    }
}
