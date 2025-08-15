use crate::passes::Pass;

pub struct PassManager {
    passes: Vec<Box<dyn Pass + Send + Sync>>,
}
impl Default for PassManager { fn default() -> Self { Self { passes: Vec::new() } } }

impl PassManager {
    pub fn add<P: Pass + Send + Sync + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }
    pub fn draw_all<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        for p in &self.passes {
            p.draw(rpass);
        }
    }
    pub fn is_empty(&self) -> bool { self.passes.is_empty() }
}
