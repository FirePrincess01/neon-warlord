//! Manages all ants instances
//!


use crate::ant_storage::Ant;

// use crate::game_logic::game_logic_interface::GameLogicMessageLight;

pub struct AntGenerator {
    pub ants: Vec<Ant>,
}

impl AntGenerator {
    pub fn new(max_nr_ants: usize) -> Self {
        let mut ants: Vec<Ant> = Vec::new();

        let size: usize = max_nr_ants.isqrt();

        let gradient = colorous::TURBO;

        let mut id = 0;
        for y in 0..size {
            for x in 0..size {
                let color0 = gradient.eval_rational(x % 10, 10);
                let color1 = gradient.eval_rational(y % 10, 10);

                let color0: cgmath::Vector3<f32> = cgmath::Vector3::new(
                    color0.r as f32 / 255.0,
                    color0.g as f32 / 255.0,
                    color0.b as f32 / 255.0,
                );

                let color1: cgmath::Vector3<f32> = cgmath::Vector3::new(
                    color1.r as f32 / 255.0,
                    color1.g as f32 / 255.0,
                    color1.b as f32 / 255.0,
                );

                let color = color0 / 2.0 + color1 / 2.0;

                ants.push(Ant {
                    id,

                    pos: cgmath::Vector2 {
                        x: x as f32 * 50.0,
                        y: y as f32 * 50.0,
                    },
                    rot_z: 0.0,
                    light_strength: 1.0,
                    // light_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
                    light_color: color,
                });

                id += 1;
            }
        }

        Self { ants }
    }



    // pub fn update(&mut self, channel: &mpsc::Sender<GameLogicMessageLight>) {
    //     // if self.requires_update {
    //     for elem in &mut self.ants {
    //         // elem.position.x += 0.02;
    //         // elem.color.x = (elem.color.x + 0.001) % 1.0;

    //         let res = channel.send(GameLogicMessageLight::UpdateAnt(elem.clone()));
    //         match res {
    //             Ok(_) => {}
    //             Err(_err) => {
    //                 // println!("{}", err)
    //             }
    //         }
    //     }
    // }
    // }
}

// #[derive(Clone)]
// pub struct Ant {
//     pub id: usize,

//     pub pos: cgmath::Vector2<f32>,
//     pub rot_z: f32,

//     pub light_strength: f32,
//     pub light_color: cgmath::Vector3<f32>,
// }
