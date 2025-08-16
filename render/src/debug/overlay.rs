use wgpu::util::DeviceExt;

pub struct UiOverlay {
    pub tex: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind: wgpu::BindGroup,
    pub bind_layout: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline,
    pub vb: wgpu::Buffer,
    pub ib: wgpu::Buffer,
    pub index_count: u32,
    pub tex_w: u32,
    pub tex_h: u32,
}

impl UiOverlay {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, tex_w: u32, tex_h: u32) -> Self {
        // 1) テクスチャ
        let tex = device.create_texture(&wgpu::TextureDescriptor{
            label: Some("ui-text"),
            size: wgpu::Extent3d{width: tex_w, height: tex_h, depth_or_array_layers:1},
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor{
            label: Some("ui-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // 2) バインド
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("ui-bind-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture{
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry{
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("ui-bind"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry{ binding:0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry{ binding:1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ]
        });

        // 3) シェーダ（頂点は NDC のフルスクリーンクアッド、描画領域は viewport で切る）
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("ui-shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("ui_textured_quad.wgsl").into()),
        });

        // 4) パイプライン
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("ui-pipeline-layout"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("ui-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout{
                    array_stride: (std::mem::size_of::<[f32;4]>()) as u64, // pos.xy, uv.xy
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0=>Float32x2, 1=>Float32x2],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // 5) 頂点（NDCフルスクリーン四角形：viewport で左上 tex_w×tex_h に制限する）
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("ui-vb"),
            contents: bytemuck::cast_slice(&[
                [-1.0f32, -1.0, 0.0, 1.0], // v: 0.0 -> 1.0 に
                [ 1.0f32, -1.0, 1.0, 1.0],
                [ 1.0f32,  1.0, 1.0, 0.0],
                [-1.0f32,  1.0, 0.0, 0.0],
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("ui-ib"),
            contents: bytemuck::cast_slice(&[0u16,1,2, 0,2,3]),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self{
            tex, view, sampler, bind, bind_layout,
            pipeline, vb, ib, index_count: 6,
            tex_w, tex_h,
        }
    }

    pub fn upload_rgba(&self, queue: &wgpu::Queue, rgba: &[u8]) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo{
                texture: &self.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout{
                offset: 0,
                // ← ここを NonZeroU32 ではなく素の u32 で渡す
                bytes_per_row: Some(self.tex_w * 4),
                rows_per_image: Some(self.tex_h),
            },
            wgpu::Extent3d{ width: self.tex_w, height: self.tex_h, depth_or_array_layers: 1 },
        );
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView, _surface_w: u32, _surface_h: u32) {
        // 左上に tex_w x tex_h だけ描く（Loadで上書きせず合成）
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
            label: Some("ui-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                view: target,
                resolve_target: None,
                depth_slice: None, // v26で追加
                ops: wgpu::Operations{ load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        rp.set_pipeline(&self.pipeline);
        rp.set_bind_group(0, &self.bind, &[]);
        rp.set_vertex_buffer(0, self.vb.slice(..));
        rp.set_index_buffer(self.ib.slice(..), wgpu::IndexFormat::Uint16);
        // Viewportを左上texサイズに設定（NDCフルスクリーンクアッドがこの範囲に描かれる）
        rp.set_viewport(0.0, 0.0, self.tex_w as f32, self.tex_h as f32, 0.0, 1.0);
        rp.draw_indexed(0..self.index_count, 0, 0..1);
    }

    pub fn ensure_width(&mut self, device: &wgpu::Device, new_w: u32) {
        if new_w <= self.tex_w { return; }
        self.tex_w = new_w;
        // テクスチャ＆ビュー再作成（パイプラインは使い回しOK）
        self.tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ui-text"),
            size: wgpu::Extent3d { width: self.tex_w, height: self.tex_h, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.view = self.tex.create_view(&wgpu::TextureViewDescriptor::default());
        self.bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui-bind"),
            layout: &self.bind_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
    }
}
