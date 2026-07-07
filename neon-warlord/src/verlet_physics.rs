//! Physics simulation using verlet

pub mod solver;

use cgmath::Zero;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

pub struct VerletObject {
    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
}

impl VerletObject {
    pub fn new(position_current: Vec2) -> Self {
        let position_old = position_current.clone();
        let acceleration = Vec2::zero();

        Self { position_current, position_old, acceleration }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;
        // Save current position
        self.position_old = self.position_current;
        // Perform Verlet integration
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        // Reset acceleration
        self.acceleration = Vec2::zero();
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }

    pub fn position(&self) -> Vec2 {
        self.position_current
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.position_current = pos;
    }
}