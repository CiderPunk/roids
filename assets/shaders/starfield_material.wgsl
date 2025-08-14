#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}



/* Hash without Sine https://www.shadertoy.com/view/4djSRW

Copyright (c) 2014 David Hoskins.
Copyright (c) 2022 David A Roberts <https://davidar.io/> (WGSL port)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


https://gist.github.com/davidar/5f9677a0ccfbd63d7a8657ad9af3a856
*/

fn hash12(p: vec2<f32>) -> f32
{
    var p3  = fract(vec3<f32>(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn hash32(p: vec2<f32>) -> vec3<f32>
{
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yxz+33.33);
    return fract((p3.xxy+p3.yzz)*p3.zyx);
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
  let point = hash12(mesh.uv * 2000. + (globals.time * 0.000001));
  if point > 0.999{
    return vec4(1.,1.,1.,1.);
  }
  else{
    return vec4(0.,0.,0.,1.);

  }
  //return vec4(point,point,point,1.);
}