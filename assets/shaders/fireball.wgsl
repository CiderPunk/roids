#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> color: vec4<f32>;

//https://www.shadertoy.com/view/XdXGW8
fn grad( v:vec2<f32>)  -> f32 
{
    // 2D to 1D  (feel free to replace by some other)
    var n = v.x+v.y*11111;
    // Hugo Elias hash (feel free to replace by another one)
    n = (n<<13)^n;
    n = (n*(n*n*15731+789221)+1376312589)>>16;

    // simple random vectors
    return vec2(cos(n),sin(n));
    
/*
    // Perlin style vectors
    n &= 7;
    vec2 gr = vec2(n&1,n>>1)*2.0-1.0;
    return ( n>=6 ) ? vec2(0.0,gr.x) : 
           ( n>=4 ) ? vec2(gr.x,0.0) :
                              gr;
*/
}

fn noise( p: vec2<f32>) -> f32
{
    let i = floor( p );
    let f = fract( p );
	
	  let u = f*f*(3.0-2.0*f); // feel free to replace by a quintic smoothstep instead

    return mix( mix( dot( grad( i+vec2(0.,0.) ), f-vec2(0.0,0.0) ), 
                     dot( grad( i+vec2(1.,0.) ), f-vec2(1.0,0.0) ), u.x),
                mix( dot( grad( i+vec2(0.,1.) ), f-vec2(0.0,1.0) ), 
                     dot( grad( i+vec2(1.,1.) ), f-vec2(1.0,1.0) ), u.x), u.y);
}



@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
//https://www.shadertoy.com/view/llXGDN
	let uv = mesh.uv;
  //let r = iResolution.xy, 
  var p = uv*.5;
    
	let d = length(p);
  var c = 4.-d*9.;
  var k=1.;
    
	p = vec2(atan(p.x,p.y), d-iDate.w/4.)*.02;       
    
  for(var i=1; i<7; i++){
    c += noise(p).r / k;
    p += p;
     k += k;
  }
    
	return vec4(.3,.15,.1,1)*c*c;
}