// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) color: vec3<f32>, 
    @location(7) time: f32, 
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex 
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let position = model.position + instance.position;

    var out: VertexOutput;
    out.color = instance.color;
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}


fn rand_xorshift(rng_state: u32) -> u32
{
    let rng_state1 = rng_state ^ (rng_state << 13);
    let rng_state2 =  rng_state1 ^ (rng_state1 >> 17);
    let rng_state3 = rng_state2 ^ (rng_state2 << 5);
    return rng_state3;
}

// “Wang hash", s a general-purpose 32-bit-to-32-bit integer hash function.
fn wang_hash(seed: u32) -> u32
{
    let seed1 = (seed ^ 61) ^ (seed >> 16);
    let seed2 = seed1 * 9;
    let seed3 = seed2 ^ (seed2 >> 4);
    let seed4 = seed3 * 0x27d4eb2d;
    let seed5 = seed4 ^ (seed4 >> 15);
    return seed5;
}

// 32-bit PCG hash used by Jarzynski and Olano, one of the best balanced choices between 
// performance and quality overall.
fn pcg_hash(input: u32) -> u32
{
    let state = input * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

// returns a random number between 0 and 1
fn random(input: u32) -> f32 {
    
    let val2 = rand_xorshift(input);
    let val3 = wang_hash(val2);
    let val4 = f32(val3) * (1.0 / 4294967296.0);
    
    return val4;
}