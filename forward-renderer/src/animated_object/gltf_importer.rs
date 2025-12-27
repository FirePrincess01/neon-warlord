use crate::animated_object::animated_object_data::{AnimationRotation, AnimationTranslation};

use super::animated_object_data::{AnimatedObjectData, AnimationData, MeshData, SkeletonData};

pub struct GltfImporter {}

impl GltfImporter {
    pub fn create(glb_bin: &[u8]) -> AnimatedObjectData {
        let (document, buffer_data, _image_data) = gltf::import_slice(glb_bin).unwrap();
        let scene = document.scenes().next().expect("No scene found!");
        let node = scene.nodes().next().expect("No Node in scene found!");
        let mesh_node = node.children().next().expect("No children found in node!");
        let mesh: gltf::Mesh<'_> = mesh_node.mesh().expect("Mesh not found in node!");
        let skin = mesh_node.skin().expect("Skin not found in node!");
        let animations = document.animations();

        let mesh_data = Self::get_mesh_data(&buffer_data, &mesh);
        let skeleton_data = Self::get_skin_data(&buffer_data, &skin);
        let animation_data =
            Self::get_animation_data(&buffer_data, animations, &skeleton_data.joint_names);

        AnimatedObjectData {
            mesh: mesh_data,
            skeleton: skeleton_data,
            animations: animation_data,
        }
    }

    fn get_mesh_data(buffer_data: &[gltf::buffer::Data], mesh: &gltf::Mesh<'_>) -> MeshData {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut joints = Vec::new();
        let mut weights = Vec::new();
        let mut indices = Vec::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

            let position_iter = reader.read_positions().expect("No positions found");
            for elem in position_iter {
                positions.push(elem);
            }

            let normals_iter = reader.read_normals().expect("No normals found");
            for elem in normals_iter {
                normals.push(elem);
            }

            let tex_coords_iter = reader.read_tex_coords(0).expect("No tex_coords found");
            match tex_coords_iter {
                gltf::mesh::util::ReadTexCoords::F32(iter) => {
                    for elem in iter {
                        tex_coords.push(elem);
                    }
                }
                _ => {
                    panic!("Only f32 is supported so far!")
                }
            }

            let joints_iter = reader.read_joints(0).expect("No joints found");
            match joints_iter {
                gltf::mesh::util::ReadJoints::U8(iter) => {
                    for elem in iter {
                        joints.push(elem);
                    }
                }
                _ => {
                    panic!("Only u8 is supported so far!")
                }
            }

            let weights_iter = reader.read_weights(0).expect("No weights found");
            match weights_iter {
                gltf::mesh::util::ReadWeights::F32(iter) => {
                    for elem in iter {
                        weights.push(elem);
                    }
                }
                _ => {
                    panic!("Only f32 is supported so far!")
                }
            }

            let indices_iter = reader.read_indices().expect("Nod indices found");
            match indices_iter {
                gltf::mesh::util::ReadIndices::U16(iter) => {
                    for elem in iter {
                        indices.push(elem);
                    }
                }
                _ => {
                    panic!("Only u16 is supported so far!")
                }
            }
        }

        MeshData {
            positions,
            normals,
            _tex_coords: tex_coords,
            joints,
            weights,
            indices,
        }
    }

    fn get_skin_data(buffer_data: &[gltf::buffer::Data], skin: &gltf::Skin<'_>) -> SkeletonData {
        let mut joint_name: Vec<String> = Vec::new();
        let mut joint_children: Vec<Vec<String>> = Vec::new();
        let mut joint_translation: Vec<cgmath::Vector3<f32>> = Vec::new();
        let mut joint_rotation: Vec<cgmath::Quaternion<f32>> = Vec::new();
        let mut inverse_bind_transform: Vec<cgmath::Matrix4<f32>> = Vec::new();

        // inverse bind transform
        let reader = skin.reader(|buffer| Some(&buffer_data[buffer.index()]));

        let inverse_bind_matrices_iter = reader.read_inverse_bind_matrices().unwrap();
        for elem in inverse_bind_matrices_iter {
            let mat = cgmath::Matrix4::from(elem);
            inverse_bind_transform.push(mat);
        }

        // name, translation, rotation
        let joints_iter = skin.joints();
        for joint in joints_iter {
            let name = joint.name().unwrap();
            let (translation, rotation, _scale) = joint.transform().decomposed();
            let _bind_transform = joint.transform().matrix();

            let children = joint.children();
            // println!("children: {}", children.clone().count());
            let mut childrent_vec: Vec<String> = Vec::new();
            for child in children {
                let child_name = child.name().unwrap();
                // println!("child name: {}", child.name().unwrap());
                childrent_vec.push(child_name.to_string());
            }

            joint_name.push(name.to_string());
            joint_children.push(childrent_vec);
            joint_translation.push(cgmath::Vector3::from(translation));
            joint_rotation.push(cgmath::Quaternion::from(rotation));
        }

        SkeletonData {
            joint_names: joint_name,
            joint_children,
            joint_translations: joint_translation,
            joint_rotations: joint_rotation,
            inverse_bind_transforms: inverse_bind_transform,
        }
    }

    fn get_animation_data(
        buffer_data: &[gltf::buffer::Data],
        animations: gltf::iter::Animations<'_>,
        joint_names: &[String],
    ) -> Vec<AnimationData> {
        let mut animation_data: Vec<AnimationData> = Vec::new();

        for animation in animations {
            let mut joint_target_names: Vec<String> = Vec::new();
            let mut joint_translations: Vec<AnimationTranslation> = Vec::new();
            let mut joint_rotations: Vec<AnimationRotation> = Vec::new();

            // gen name
            let animation_name = animation.name().unwrap();
            // println!("animation name: {}", name);

            let mut channesls_iter = animation.channels();
            let channels_len = channesls_iter.clone().count();
            let _length = joint_names.len();

            #[allow(clippy::needless_range_loop)]
            for i in 0..channels_len / 3 {
                let channel_translate = channesls_iter.next().unwrap();
                let channel_rotate = channesls_iter.next().unwrap();
                let channel_scale = channesls_iter.next().unwrap();

                // make sure the bones are exactly in the right order
                let name = &joint_names[i];
                assert_eq!(name, channel_translate.target().node().name().unwrap());
                assert_eq!(name, channel_rotate.target().node().name().unwrap());
                assert_eq!(name, channel_scale.target().node().name().unwrap());
                joint_target_names.push(name.to_string());

                let reader_translate =
                    channel_translate.reader(|buffer| Some(&buffer_data[buffer.index()]));
                let reader_rotate =
                    channel_rotate.reader(|buffer| Some(&buffer_data[buffer.index()]));

                // get translation
                let mut animation_translation = AnimationTranslation {
                    key_times: Vec::new(),
                    joint_translations: Vec::new(),
                };
                let output_translation = reader_translate.read_outputs().unwrap();
                match output_translation {
                    gltf::animation::util::ReadOutputs::Translations(iter) => {
                        animation_translation
                            .joint_translations
                            .push(cgmath::Vector3::from(iter.clone().next().unwrap()));
                    }
                    _ => panic!("Expected Translation Element!"),
                }
                let input_transloation = reader_translate.read_inputs().unwrap();
                for elem in input_transloation {
                    animation_translation.key_times.push(elem);
                }

                // get rotations
                let mut animation_rotation = AnimationRotation {
                    key_times: Vec::new(),
                    joint_rotations: Vec::new(),
                };
                let output_rotation = reader_rotate.read_outputs().unwrap();
                match output_rotation {
                    gltf::animation::util::ReadOutputs::Rotations(rotations) => match rotations {
                        gltf::animation::util::Rotations::F32(iter) => {
                            for elem in iter {
                                animation_rotation
                                    .joint_rotations
                                    .push(cgmath::Quaternion::from(elem));
                            }
                        }
                        _ => {
                            panic!("Rotation is only implemented for Quaternion<f32>")
                        }
                    },
                    _ => panic!("Expected Rotation Element!"),
                }

                let input_rotation = reader_rotate.read_inputs().unwrap();
                for elem in input_rotation {
                    animation_rotation.key_times.push(elem);
                }

                joint_translations.push(animation_translation);
                joint_rotations.push(animation_rotation);
            }

            let animation_data_element = AnimationData {
                _name: animation_name.to_string(),
                _joint_target_names: joint_target_names,
                joint_translations,
                joint_rotations,
            };

            animation_data.push(animation_data_element);
        }

        animation_data
    }
}
