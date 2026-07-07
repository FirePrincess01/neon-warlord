//! Solves collision for verlet physics

use cgmath::{InnerSpace, MetricSpace};

use crate::verlet_physics::VerletObject;

type Vec2 = cgmath::Vector2<f32>;

pub struct Solver {
    
}

impl Solver {
    pub fn new() -> Self {
        Self {  }
    }
    
    pub fn update(&mut self, verlet_objects: &mut [VerletObject], dt: f32) {
        Self::apply_gravity(verlet_objects);
        Self::apply_constraint(verlet_objects);
        Self::solve_collisions(verlet_objects);
        Self::update_positions(verlet_objects, dt);
    }

    fn update_positions(verlet_objects: &mut [VerletObject], dt: f32) {
        for elem in verlet_objects {
            elem.update_position(dt);
        }
    }

    fn apply_gravity(verlet_objects: &mut [VerletObject]) {
        const GRAVITY: Vec2 = Vec2::new(0.0, -10.0);

        for elem in verlet_objects {
            elem.accelerate(GRAVITY);
        }
    }

    fn apply_constraint(verlet_objects: &mut [VerletObject]) {
        const POSITION: Vec2 = Vec2::new(-3.0, 10.0);
        const RADIUS: f32 = 10.0;

        for elem in verlet_objects {
            let to_obj = elem.position() - POSITION;
            // let dist = elem.position().distance(POSITION);
            let dist = POSITION.distance(elem.position());

            // println!("dist {dist}");

            if dist > RADIUS - 0.5 {
                let n = to_obj / dist;
                let new_pos = POSITION + n * (RADIUS - 0.5);
                // println!("POSITION {} {}",POSITION.x, POSITION.y);
                // println!("elem.position {} {}",elem.position().x, elem.position().y);
                // println!("n {} {}",n.x, n.y);
                // println!("n {} {}",n.x, n.y);
                // println!("dist {dist}");
                // println!("dist {dist}");
                // println!("new_pos {} {}",new_pos.x, new_pos.y);

                elem.set_position(new_pos);

            }
        }
    }

    fn solve_collisions(verlet_objects: &mut [VerletObject]) {
        let object_count = verlet_objects.len();

        for i in 0..object_count {
            for k in i+1..object_count {
                let (left, right) = verlet_objects.split_at_mut(k);

                let object_1 = &mut left[i];
                let object_2 = &mut right[0];


                let collision_axis = object_1.position() - object_2.position();
                let dist = collision_axis.magnitude();
                if dist < 1.0 {
                    let n = collision_axis / dist;
                    let delta = 1.0 - dist;
                    object_1.set_position(object_1.position() + 0.5 * delta * n);
                    object_2.set_position(object_2.position() - 0.5 * delta * n);
                }
            }
        }
    }
}