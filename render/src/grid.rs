use wgpu::{Device, Buffer, RenderPipeline, SurfaceConfiguration, RenderPass};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

/// グリッド線の頂点（3D座標のみ）
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GridVertex {
    pub position: [f32; 3],
}

/// グリッド描画用構造体
pub struct Grid {
    pub vertex_buffer: Buffer,
    pub vertex_count: u32,
    pub pipeline: RenderPipeline,
}

impl Grid {
    /// グリッドを生成（例：XY平面に10x10の格子）
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        // グリッド線の頂点生成
        let mut vertices = Vec::new();
        let size: f32 = 10.0;
        let step: f32 = 1.0;
        let half = size / 2.0;

        // X軸方向の線
        for i in 0..=size as i32 {
            let pos = -half + i as f32 * step;
            vertices.push(GridVertex { position: [pos, -half, 0.0] });
            vertices.push(GridVertex { position: [pos, half, 0.0] });
        }
        // Y軸方向の線
        for i in 0..=size as i32 {
            let pos = -half + i as f32 * step;
            vertices.push(GridVertex { position: [-half, pos, 0.0] });
            vertices.push(GridVertex { position: [half, pos, 0.0] });
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // シェーダー（grid.wgsl）読み込み
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Grid Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/grid.wgsl").into()),
        });

        // パイプライン作成
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Grid Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Grid Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GridVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    }],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            vertex_buffer,
            vertex_count: vertices.len() as u32,
            pipeline,
        }
    }

    /// グリッド描画
    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }
}
