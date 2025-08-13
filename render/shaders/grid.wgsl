// 頂点シェーダ
struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // 単純な正射影（XY平面、Z=0）
    out.position = vec4<f32>(input.position, 1.0);
    return out;
}

// フラグメントシェーダ
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // グリッド線の色（白）
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
