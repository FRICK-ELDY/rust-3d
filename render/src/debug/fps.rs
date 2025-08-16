#[cfg(target_arch = "wasm32")]
use web_time::{Instant, Duration};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, Duration};

pub struct FpsCounter {
    last: Instant,
    pub frame_ms: f32,
    pub fps: f32,
    ema_ms: f32,
    alpha: f32,
    acc: Duration,
    frames: u32,
}
impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            frame_ms: 0.0,
            fps: 0.0,
            ema_ms: 0.0,
            alpha: 0.15,
            acc: Duration::from_secs(0), 
            frames: 0,
        }
    }
    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now - self.last;
        self.last = now;

        let ms = dt.as_secs_f32() * 1000.0;
        self.ema_ms = if self.ema_ms == 0.0 { ms } else { self.alpha * ms + (1.0 - self.alpha) * self.ema_ms };
        self.frame_ms = self.ema_ms;

        self.acc += dt;
        self.frames += 1;
        if self.acc.as_secs_f32() >= 0.5 {
            self.fps = self.frames as f32 / self.acc.as_secs_f32();
            self.frames = 0;
            self.acc = Duration::ZERO;
        }
    }
}
