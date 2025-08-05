#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> color: vec4<f32>;



/*https://www.shadertoy.com/view/Dsc3R4
 * Fast FBM Fire
 * Copyright (C) 2023 NR4 <nr4@z10.info>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
 
const c = vec3(1, 0, -1);
const  m = .4 * mat2x2(4., 3., -3., 4.);

// Created by David Hoskins and licensed under MIT.
// See https://www.shadertoy.com/view/4djSRW.
fn hash12(p:vec2<f32>) -> f32 
{
	let p3  = fract(vec3(p.xyx) * .1031);
  let p4 = p3 + dot(p3, p3.yzx + 33.33);
  return fract(dot(p4.xy, p4.zz));
}

fn lfnoise(t:vec2<f32>) -> f32
{
    let i = floor(t);
    let t2 = c.xx * smoothstep(0., 1., fract(t));
    let v1 = 2. * mix(vec2(hash12(i), hash12(i + c.xy)), vec2(hash12(i + c.yx), hash12(i + c.xx)), t2.y) - 1.;
    return mix(v1.x, v1.y, t.x);
}

fn fbm(uv:vec2<f32>)  ->f32
{

    let time= globals.time;
    //let uv0 = uv;
    var uv1 = uv * vec2(5., 2.) - vec2(-2., -.25) - 3.1 * time * c.yx;
	  var f = 1.;
    var a = .5;
    var c = 2.5;
	
    for(var i = 0; i < 5; i+=1) {
      uv1.x -= .15 * clamp(1. - pow(uv.y, 4.), 0., 1.) * lfnoise(c * (uv1 + f32(i) * .612 + time));
      c *= 2.;
      f += a * lfnoise(uv1 + f32(i) * .415);
      a /= 2.;
      uv1 *= m;
    }
    return f / 2.;
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    return vec4(clamp(1.5 * pow(clamp(pow(fbm(uv), 1. + 4. * clamp(uv.y * uv.y, 0., 1.)) * 1.5, 0., 1.) * c.xxx, vec3(1, 3, 6)), 0., 1.), 1);
}

  