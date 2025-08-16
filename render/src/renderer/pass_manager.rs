use crate::passes::Pass;

#[derive(Default)]
pub struct PassManager {
    passes: Vec<Box<dyn Pass + Send + Sync>>,
}

impl PassManager {
    pub fn add<P: Pass + Send + Sync + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }
    pub fn draw_all<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        for p in &self.passes {
            p.draw(rpass);
        }
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }
}
