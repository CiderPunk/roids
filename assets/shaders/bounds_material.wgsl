#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = 0.25 / mesh.uv ;
    let pi = 3.141592;
    let time= globals.time;
    
    
    //p *= 5. / uv.xy;

    let a = vec4(.1,.4,.222,0) + time + atan2(uv.y, uv.x);
    let b = vec4(a.x, a.y+.4, a.z, a.w); 
    
    
    let c = cos(sin(uv.x)-cos(uv.y) +a );
    let d = sin(c*uv.x*uv.y - uv.y   +b );
    let e =  abs(d*d-c*c);

    
    return 1.6 * pow(1.-e+e*e,  16.+e-e);
}