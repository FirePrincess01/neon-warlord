
const MAX_JOINTS: u32 = 16u;
const MAX_JOINT_WEIGHTS: u32 = 4u;

// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

struct JointUniform {
    joint_transform: array<mat4x4<f32>, 16>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> joints: JointUniform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,

    // animation data
    @location(2) joint_indices: vec4<u32>,
    @location(3) joint_weights: vec4<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) color: vec3<f32>,
    // @location(7) entity: vec3<u32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) normal: vec3<f32>,
    // @location(3) entity: vec3<u32>,
};

@vertex 
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    // calculate the animation
    var total_local_pos = vec4<f32>(0.0);
    var total_local_normal = vec4<f32>(0.0);

    for (var i: u32 = 0u; i < MAX_JOINT_WEIGHTS; i++) {
        // get vertex data
        let joint_index = model.joint_indices[i];
        let joint_weight = model.joint_weights[i];

        // get transform matrix
        let joint_transform = joints.joint_transform[joint_index];

        // move position
        let local_position = joint_transform * model.position;
        total_local_pos += local_position * joint_weight;

        // move normal
        let local_normal = joint_transform * model.normal;
        total_local_normal += local_normal * joint_weight;
    }

    // move to the instance position
    let world_position = instance.position + total_local_pos.xyz;

    // calculate lighting
    // let light_intensity = 0.8;
    // let light_direction = normalize(vec3<f32>(1.0, -0.1, 1.0));

    // let diffuse_lighting = clamp(dot(total_local_normal.xyz, light_direction) * light_intensity, 0.0, 1.0);
    // let color = instance.color * diffuse_lighting;
    let color = instance.color;

    // apply camera
    let clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);

    // calculate output
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = color;
    out.position = world_position;
    out.normal = total_local_normal.xyz;
    // out.entity = instance.entity;

    return out;
}

// Fragment shader
struct FragmentOutput {
    @location(0) surface: vec4<f32>,
    // @location(0) position: vec4<f32>,
    // @location(1) normal: vec4<f32>,
    // @location(2) albedo: vec4<f32>,
    // @location(3) entity: vec4<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {

    // var entity0 = (in.entity[0] >> 0u) & 0xffu;
    // var entity1 = (in.entity[0] >> 8u) & 0xffu;
    // var entity2 = (in.entity[0] >> 16u) & 0xffu;
    // var entity3 = (in.entity[0] >> 24u) & 0xffu;

    // var out: FragmentOutput;
    // out.surface = vec4<f32>(in.color, 0.8);
    // out.position =  vec4<f32>(in.position, 1.0);
    // out.normal =  vec4<f32>(in.normal, 1.0);
    // out.albedo = vec4<f32>(in.color, 1.0);
    // out.entity =  vec4<f32>(
    //     f32(entity0)/255.0, 
    //     f32(entity1)/255.0, 
    //     f32(entity2)/255.0, 
    //     f32(entity3)/255.0);

    let color_out = vec4<f32>(in.color, 1.0);

    var out: FragmentOutput;
    out.surface = color_out;

    return out;
}
