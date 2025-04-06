struct Vertex {
    @location(0) position: vec3<f32>,
}

struct OutputVertex {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vertex(vertex: Vertex) {
    var out: OutputVertex;
    out.clip_position = vec4(vertex.position.xyz, 1.0);

    return out;
}

@fragment
fn fragment(output_vertex: OutputVertex) -> @location(0) vec4<f32> {
    return output_vertex.clip_position;
}
