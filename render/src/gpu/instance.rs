use anyhow::Result;

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter:  wgpu::Adapter,
    pub device:   wgpu::Device,
    pub queue:    wgpu::Queue,
}

impl GpuContext {
    pub async fn with_surface(compatible_surface: &wgpu::Surface<'static>) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(compatible_surface),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("render/device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                    ..Default::default()
                }
            )
            .await?;

        Ok(Self { instance, adapter, device, queue })
    }

    pub async fn headless() -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
        let (device, queue) =
            adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        Ok(Self { instance, adapter, device, queue })
    }
}
