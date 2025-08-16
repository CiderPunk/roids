#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals,

struct AnimationSettings{
  frame_rate:f32,
  frames_wide:f32, 
  frames_deep:f32,
  frame_count:f32,
  //init_time:f32,
}
@group(2) @binding(0) var<uniform> settings: AnimationSettings;
@group(2) @binding(1) var atlas_texture: texture_2d<f32>;
@group(2) @binding(2) var atlas_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  //get tag time which is the animstart time f32 bitcast to u32
  let tag = mesh_functions::get_tag(mesh.instance_index);
  //convert it back to f32
  let start_time = bitcast<f32>(tag);

  let frame = floor((globals.time - start_time) * settings.frame_rate);
  if frame > setting.frame_count{
    return (1.,0.,0.,1.);
  }
  let atlas_uv = vec2<f32>(
    ((frame % settings.frames_wide) * (1./settings.frames_wide)) + (mesh.uv.x / settings.frames_wide),
    (floor(frame/settings.frames_deep) * 1./settings.frames_deep) + (mesh.uv.y / settings.frames_deep),
  );
  return textureSample(atlas_texture, atlas_sampler, atlas_uv);
}