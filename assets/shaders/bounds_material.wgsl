#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

    let f = mesh.world_position.xz * (1.0 / 5.0);
    let s = sin(f.x+f.y + globals.time * 20.0);
    return ((1.0-mesh.uv.y) * s * color) + ((1.0-mesh.uv.y) * color * 1.);
}