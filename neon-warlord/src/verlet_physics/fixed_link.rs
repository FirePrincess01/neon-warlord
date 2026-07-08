//! A link between two objects with fixed angle

use crate::verlet_physics::{Vec3, VerletObject};

pub struct FixedLink {
    node_id_1: usize,
    node_id_2: usize,
    target_position: Vec3,
}

impl FixedLink {
    pub fn new(node_id_2: usize, node_id_1: usize, target_position: Vec3) -> Self {
        Self {
            node_id_1,
            node_id_2,
            target_position,
        }
    }

    pub fn apply(&self, verlet_objects: &mut [VerletObject]) {
        if self.node_id_1 >= verlet_objects.len()
            || self.node_id_2 >= verlet_objects.len()
            || self.node_id_1 == self.node_id_2
        {
            return;
        }

        let (object_1, object_2) = if self.node_id_1 < self.node_id_2 {
            let (left, right) = verlet_objects.split_at_mut(self.node_id_2);
            (&mut left[self.node_id_1], &mut right[0])
        } else {
            let (left, right) = verlet_objects.split_at_mut(self.node_id_1);
            (&mut right[0], &mut left[self.node_id_2])
        };

        let axis = self.target_position - (object_1.position() - object_2.position());

        let elasticity = 0.9;

        object_1.set_position(object_1.position() + 0.5 * axis * elasticity);
        object_2.set_position(object_2.position() - 0.5 * axis * elasticity);
    }
}
