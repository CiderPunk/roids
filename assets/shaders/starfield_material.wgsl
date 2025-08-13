#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}


// Creative Commons Attribution-ShareAlike 4.0 International Public License
// Created by David Hoskins. May 2018
// https://www.shadertoy.com/view/XdGfRR
#define UI0 1597334673U
#define UI1 3812015801U
#define UI2 uvec2(UI0, UI1)
#define UIF (1.0 / float(0xffffffffU))
fn hash22A(p: vec2<f32>) -> vec2<f32> 
{
	var q = uvec2(ivec2(p))*UI2;
	q = (q.x ^ q.y) * UI2;
	return vec2(q) * UIF;
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let test = hash22A(mesh.uv);
   return vec4(fract(test), 0.5,1);
}