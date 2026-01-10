//! A position which can be interpolated over its last position

use cgmath::{VectorSpace, Zero};

#[derive(Clone, Copy, Debug)]
pub struct InterpolatedPosition {
    last_pos: cgmath::Vector3<f32>,
    last_time_stamp: instant::Instant,

    pub pos: cgmath::Vector3<f32>,
    time_stamp: instant::Instant,
}

impl InterpolatedPosition {
    pub fn zero() -> Self {
        let last_pos = cgmath::Vector3::zero();
        let pos = cgmath::Vector3::zero();
        let now = instant::Instant::now();
        let last_time_stamp = now;
        let time_stamp = now;

        Self {
            last_pos,
            last_time_stamp,
            pos,
            time_stamp,
        }
    }

    pub fn _new(pos: cgmath::Vector3<f32>, time_stamp: instant::Instant) -> Self {
        let last_pos = pos;
        let last_time_stamp = time_stamp;

        Self {
            last_pos,
            last_time_stamp,
            pos,
            time_stamp,
        }
    }

    pub fn lerp(&self, time_stamp: instant::Instant) -> cgmath::Vector3<f32> {
        let amount = (time_stamp - self.last_time_stamp).as_nanos() as f32
            / (self.time_stamp - self.last_time_stamp).as_nanos() as f32;

        

        self.last_pos.lerp(self.pos, amount)
    }

    pub fn add(&mut self, pos: cgmath::Vector3<f32>, time_stamp: instant::Instant) {
        self.last_pos = self.pos;
        self.last_time_stamp = self.time_stamp;

        self.pos = pos;
        self.time_stamp = time_stamp;
    }
}
