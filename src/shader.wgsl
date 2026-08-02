struct VertexInput {
	@location(0) position: vec2f,
	@location(1) uv: vec2f,
	@location(2) color: vec4f,
};

struct VertexOutput {
	@builtin(position) clip_position: vec4f,
	@location(0) color: vec4f,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
	return VertexOutput(vec4f(model.position * vec2f(0.00104166667, -0.00185185185) + vec2f(-1.0, 1.0), 0.0, 1.0), model.color);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
	return in.color;
}
