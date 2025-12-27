// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var t_heightmap: texture_2d<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) color: vec3<f32>, 
    // @location(7) entity: u32,
    @location(7) distance: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) normal: vec3<f32>,
    // @location(3) entity: u32,
    @location(3) tex_coords: vec2<f32>,
};

@vertex 
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let dim: vec2<u32> = textureDimensions(t_heightmap);
    // let index = vec2<u32>(vertex_index % dim.x, vertex_index / dim.y);
    let index = vec2<u32>(u32(model.position.x), u32(model.position.y));
    let distance = instance.distance;
    let tex_coords = vec2<f32>(model.position.x * distance, model.position.y * distance);
    // let tex_coords = vec2<f32>(model.position.x, model.position.y);

    let heights = get_neighborhood(index);

    let pos_rgb: vec4<f32> = textureLoad(t_heightmap, index, 0);
    let posz = pos_rgb.r;

    // calculate position
    let vertex_position = vec3<f32>(model.position.x * distance, model.position.y *distance, posz);
    let position = instance.position + vertex_position;

    // normal calculation, use negative derivatives
    let normal_x = (heights.w - heights.e) / 2.0;
    let normal_y = (heights.s - heights.n) / 2.0;
    let normal = normalize(vec3<f32>(normal_x, normal_y, distance));

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    out.color = instance.color;
    out.position = position;
    out.normal = normal;
    // out.entity = instance.entity;
    out.tex_coords = tex_coords;

    return out;
}

// Fragment shader
@group(1) @binding(0)
var t_texture: texture_2d<f32>;
@group(1) @binding(1)
var s_texture: sampler;

// struct FragmentOutput {
//     // @location(0) surface: vec4<f32>,
//     @location(0) position: vec4<f32>,
//     @location(1) normal: vec4<f32>,
//     @location(2) albedo: vec4<f32>,
//     // @location(3) entity: vec4<f32>,
// };

struct FragmentOutput {
    @location(0) surface: vec4<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {

    // var entity0 = (in.entity >> 0u) & 0xffu;
    // var entity1 = (in.entity >> 8u) & 0xffu;
    // var entity2 = (in.entity >> 16u) & 0xffu;
    // var entity3 = (in.entity >> 24u) & 0xffu;

    let color = textureSample(t_texture, s_texture, in.tex_coords);
    // let color_out = vec4<f32>(in.color.xyz, color[3]);
    let color_out = vec4<f32>(in.color.xyz * color[3], color[3]);

    var out: FragmentOutput;
    out.surface = color_out;
    // out.position =  vec4<f32>(in.position, 1.0);
    // out.normal =  vec4<f32>(in.normal, 1.0);
    // out.albedo = vec4<f32>(color.xyz, 0.5);
    // out.albedo = color;
    // out.albedo = color_out;
    // out.entity =  vec4<f32>(
    //     f32(entity0)/255.0, 
    //     f32(entity1)/255.0, 
    //     f32(entity2)/255.0, 
    //     f32(entity3)/255.0);

    return out;
}

// structure for getting the neighboring values of the height texture
struct Neighborhood {
    m: f32,
    n: f32,
    e: f32,
    s: f32,
    w: f32,
}

fn get_neighborhood(uv: vec2<u32>) -> Neighborhood
{
    let m = get_height(uv, 0, 0);
    let n = get_height(uv, 0, 1);
    let e = get_height(uv, 1, 0);
    let s = get_height(uv, 0, -1);
    let w = get_height(uv, -1, 0);

    return Neighborhood(
        m,
        n,
        e,
        s,
        w,
    );
}

fn get_height(uv: vec2<u32>, u_offset: i32, v_offset: i32) -> f32
{
    let u: u32 = u32(i32(uv.x) + u_offset);
    let v: u32 = u32(i32(uv.y) + v_offset);

    return textureLoad(t_heightmap, vec2(u, v), 0).r; 
}