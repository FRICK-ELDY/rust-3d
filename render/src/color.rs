pub struct ColorUtils;

impl ColorUtils {
    /// 脈動する色を生成する
    /// 
    /// # Arguments
    /// * `base` - 基本のRGBA色（[f32; 4]）
    /// * `t` - 時間やフレームカウンタなどの値
    /// 
    /// # Returns
    /// 脈動を加えたRGBA色（[f32; 4]）
    pub fn pulse_color(base: [f32; 4], t: f32) -> [f32; 4] {
        let pulse = (t.sin() * 0.2).max(-0.2);
        [
            (base[0] + pulse).clamp(0.0, 1.0),
            base[1].clamp(0.0, 1.0),
            base[2].clamp(0.0, 1.0),
            base[3].clamp(0.0, 1.0),
        ]
    }

    /// RGBA配列からwgpu::Colorへ変換
    pub fn to_wgpu_color(rgba: [f32; 4]) -> wgpu::Color {
        wgpu::Color {
            r: rgba[0] as f64,
            g: rgba[1] as f64,
            b: rgba[2] as f64,
            a: rgba[3] as f64,
        }
    }
}
