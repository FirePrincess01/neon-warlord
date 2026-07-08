//! Solves collision for verlet physics

use cgmath::InnerSpace;

use crate::verlet_physics::{self, Vec3, VerletObject};

pub struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        verlet_objects: &mut [VerletObject],
        links: &[verlet_physics::link::Link],
        fixed_links: &[verlet_physics::fixed_link::FixedLink],
        fixed: &[verlet_physics::fixed::Fixed],
        dt: f32,
    ) {
        let sub_steps = 3;
        let sub_dt = dt / sub_steps as f32;

        for _i in 0..sub_steps {
            Self::apply_gravity(verlet_objects);
            Self::apply_constraint(verlet_objects);
            for elem in fixed {
                elem.apply(verlet_objects);
            }
            for link in links {
                link.apply(verlet_objects);
            }
            for link in fixed_links {
                link.apply(verlet_objects);
            }
            // Self::solve_collisions(verlet_objects);
            Self::update_positions(verlet_objects, sub_dt);
        }
    }

    fn update_positions(verlet_objects: &mut [VerletObject], dt: f32) {
        for elem in verlet_objects {
            elem.update_position(dt);
        }
    }

    fn apply_gravity(verlet_objects: &mut [VerletObject]) {
        const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -10.0);

        for elem in verlet_objects {
            elem.accelerate(GRAVITY);
        }
    }

    fn apply_constraint(verlet_objects: &mut [VerletObject]) {
        const POSITION: Vec3 = Vec3::new(-3.0, 0.0, 10.0);
        const RADIUS: f32 = 10.0;

        for elem in verlet_objects {
            let to_obj = elem.position() - POSITION;
            let dist = to_obj.magnitude();

            if dist > RADIUS - elem.radius() {
                let n = to_obj / dist;
                let new_pos = POSITION + n * (RADIUS - elem.radius());

                elem.set_position(new_pos);
            }
        }
    }

    fn solve_collisions(verlet_objects: &mut [VerletObject]) {
        let object_count = verlet_objects.len();

        for i in 0..object_count {
            for k in i + 1..object_count {
                let (left, right) = verlet_objects.split_at_mut(k);

                let object_1 = &mut left[i];
                let object_2 = &mut right[0];

                let collision_axis = object_1.position() - object_2.position();
                let dist = collision_axis.magnitude();
                let min_dist = object_1.radius + object_2.radius;
                if dist < min_dist {
                    let n = collision_axis / dist;
                    let delta = min_dist - dist;
                    object_1.set_position(object_1.position() + 0.5 * delta * n);
                    object_2.set_position(object_2.position() - 0.5 * delta * n);
                }
            }
        }
    }
}
