struct VsOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>, @location(1) uv: vec2<f32>) -> VsOut {
  // pos はピクセル座標（0,0）が左上、これをNDCに変換する
  // ここではビューポートを tex サイズにしているのでそのまま 0..tex を 0..viewport に対応
  var out: VsOut;
  // ここで NDC を直接使わず、RenderPass の viewport に依存
  // wgpu は set_viewport で座標系を決めるので pos はそのままでOK
  out.pos = vec4<f32>(pos, 0.0, 1.0);
  out.uv = uv;
  return out;
}

@group(0) @binding(0) var u_tex: texture_2d<f32>;
@group(0) @binding(1) var u_smp: sampler;

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
  let c = textureSample(u_tex, u_smp, in.uv);
  return c;
}
