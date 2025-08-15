use super::Pass;

pub struct GridPass;

impl GridPass {
    pub fn new() -> Self { Self }
}

impl Pass for GridPass {
    fn draw<'a>(&'a self, _pass: &mut wgpu::RenderPass<'a>) {
        // TODO: パイプライン/バッファをバインドして draw_* 呼び出し
        // ここは雛形なので何もしない
    }
}
