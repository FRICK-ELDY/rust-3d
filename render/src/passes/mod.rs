pub trait Pass {
    fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>);
}
