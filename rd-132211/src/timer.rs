use std::time::Instant;

const NS_PER_SECOND: i64 = 1000000000;
const MAX_NS_PER_UPDATE: i64 = 1000000000;
const MAX_TICKS_PER_UPDATE: u32 = 100;

pub struct Timer {
    ticks_per_second: f32,
    last_time: Instant,
    pub ticks: u32,
    pub a: f32,
    pub time_scale: f32,
    pub fps: f32,
    pub passed_time: f32,
}

impl Timer {
    pub fn new(ticks_per_second: f32) -> Timer {
        Timer {
            ticks_per_second,
            last_time: Instant::now(),
            ticks: 0,
            a: 0.0,
            time_scale: 1.0,
            fps: 0.0,
            passed_time: 0.0,
        }
    }

    pub fn advance_time(&mut self) {
        let now = Instant::now();
        let mut passed_ns = now.duration_since(self.last_time).as_nanos() as i64;
        self.last_time = now;
        if passed_ns > MAX_NS_PER_UPDATE {
            passed_ns = MAX_NS_PER_UPDATE;
        }
        self.fps = NS_PER_SECOND as f32 / passed_ns as f32;
        self.passed_time +=
            passed_ns as f32 * self.time_scale * self.ticks_per_second / NS_PER_SECOND as f32;
        self.ticks = self.passed_time as u32;
        if self.ticks > MAX_TICKS_PER_UPDATE {
            self.ticks = MAX_TICKS_PER_UPDATE;
        }
        self.passed_time -= self.ticks as f32;
        self.a = self.passed_time;
    }
}
