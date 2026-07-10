//! Physics simulation using verlet

pub mod fixed;
pub mod link;
pub mod fixed_link;
pub mod solver;

use cgmath::Zero;

type Vec3 = cgmath::Vector3<f32>;

pub struct VerletObject {
    position_current: Vec3,
    position_old: Vec3,
    acceleration: Vec3,
    radius: f32,

    // position_delta: Vec3,
}

impl VerletObject {
    pub fn new(position_current: Vec3, radius: f32) -> Self {
        let position_old = position_current;
        let acceleration = Vec3::zero();

        Self {
            position_current,
            position_old,
            acceleration,
            radius,

            // position_delta: Vec3::zero(),
        }
    }

    pub fn reset_position(&mut self, position: Vec3) {
        self.position_current = position;
        self.position_old = position;
    }

    pub fn update_position(&mut self, dt: f32) {
        // self.position_current += self.position_delta;
        // self.position_delta = Vec3::zero();

        let velocity = self.position_current - self.position_old;
        // Save current position
        self.position_old = self.position_current;
        // Perform Verlet integration
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        // Reset acceleration
        self.acceleration = Vec3::zero();
    }

    pub fn accelerate(&mut self, acc: Vec3) {
        self.acceleration += acc;
    }

    pub fn position(&self) -> Vec3 {
        self.position_current
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position_current = pos;
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn damp(&mut self, val: f32) {
        let velocity = self.position_current - self.position_old;
        let velocity = velocity * val;
        self.position_old = self.position_current - velocity;
    }

    // pub fn add_position_delta(&mut self, position_delta: Vec3) {
    //     self.position_delta += position_delta;
    // }
}
