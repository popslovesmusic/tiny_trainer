// Chromatic mix operation - additive coherence
@group(0) @binding(0) var<storage, read> tensor_a: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> tensor_b: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> output: array<vec4<f32>>;

@compute @workgroup_size(8, 8, 1)
fn chromatic_mix(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x + id.y * 8u;
    let a = tensor_a[idx];
    let b = tensor_b[idx];

    // Additive blend and normalize
    let mixed = normalize(a.rgb + b.rgb);
    let certainty = (a.w + b.w) * 0.5;

    output[idx] = vec4<f32>(mixed, certainty);
}
