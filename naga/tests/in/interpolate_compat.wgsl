// NOTE: This is basically the same as `interpolate.wgsl`, except for the removal of
// `@interpolate(flat, first)`, which is unsupported in GLSL and `compat`.

// NOTE: invalid combinations are tested in the
// `validation::incompatible_interpolation_and_sampling_types` test.
struct FragmentInput {
  @builtin(position) position: vec4<f32>,
  @location(0) @interpolate(flat) _flat : u32,
  // NOTE: not supported in `compat` or GLSL
  // // @location(1) @interpolate(flat, first) flat_first : u32,
  @location(2) @interpolate(flat, either) flat_either : u32,
  @location(3) @interpolate(linear) _linear : f32,
  @location(4) @interpolate(linear, centroid) linear_centroid : vec2<f32>,
  @location(6) @interpolate(linear, sample) linear_sample : vec3<f32>,
  @location(7) @interpolate(perspective) perspective : vec4<f32>,
  @location(8) @interpolate(perspective, centroid) perspective_centroid : f32,
  @location(9) @interpolate(perspective, sample) perspective_sample : f32,
}

@vertex
fn vert_main() -> FragmentInput {
   var out: FragmentInput;

   out.position = vec4<f32>(2.0, 4.0, 5.0, 6.0);
   out._flat = 8u;
   // out.flat_first = 9u;
   out.flat_either = 10u;
   out._linear = 27.0;
   out.linear_centroid = vec2<f32>(64.0, 125.0);
   out.linear_sample = vec3<f32>(216.0, 343.0, 512.0);
   out.perspective = vec4<f32>(729.0, 1000.0, 1331.0, 1728.0);
   out.perspective_centroid = 2197.0;
   out.perspective_sample = 2744.0;

   return out;
}

@fragment
fn frag_main(val : FragmentInput) { }
