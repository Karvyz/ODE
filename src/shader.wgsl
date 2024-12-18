var<private> VERTICES: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
	vec2<f32>(0.0, 2.0),
	vec2<f32>(1.7321, -1.0),
	vec2<f32>(-1.7321, -1.0),
);

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
		@location(0) local_position: vec2<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

@vertex
fn vs_main(    
		instance: InstanceInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
		let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(x, y, 0.0, 1.0);
		out.local_position = VERTICES[in_vertex_index];
    return out;
}


// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
		let distance = dot(in.local_position, in.local_position);
		if distance > 1.0 {
				discard;
		}
		// if distance > 0.9 {
		// 		return vec4<f32>(0.0, 0.0, 0.0, 1.0);
		// }
    return vec4<f32>(1.0);
}
