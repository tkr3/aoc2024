// https://github.com/gfx-rs/wgpu/blob/trunk/examples/src/hello_compute/shader.wgsl

@group(0)
@binding(0)
var<storage, read_write> v_numbers: array<u32>; // this is used as both input and output for convenience

fn calculate(number: u32) -> u32 {
    var n: u32 = number;

    n = prune(mix(n, n << 6u));
    n = prune(mix(n, n >> 5u));
    n = prune(mix(n, n << 11u));

    return n;
}

fn mix(a: u32, b: u32) -> u32 {
    return a ^ b;
}

fn prune(a: u32) -> u32 {
    return a % 16777216u;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var n = v_numbers[global_id.x];

    for (var i = 0u; i < 2000u; i += 1u) {
        n = calculate(n);
    }

    v_numbers[global_id.x] = n;
}
