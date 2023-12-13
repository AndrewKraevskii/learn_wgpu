// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.3, 0.9, 1.0);
}

@fragment
fn fs_main2(in: VertexOutput) -> @location(0) vec4<f32> {
    var in2 = in;
    let scale = 20.0;
    in2.vert_pos.x += 1.0;
    in2.vert_pos.y += 1.0;

    in2.vert_pos.x *= scale;
    in2.vert_pos.y *= scale;
    return vec4<f32>(sin(in2.vert_pos.x), sin(in2.vert_pos.y), 0.0, 1.0);
}
