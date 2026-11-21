// TODO Reviewed ??-??-2026.
// A separable 5-tap Gaussian blur with clamped edges and one final rounding.
struct Image {
    width: u32,
    height: u32,
    padding: vec2<u32>,
}

@group(0) @binding(0) var<uniform> image: Image;
@group(0) @binding(1) var<storage, read> source: array<u32>;
@group(0) @binding(2) var<storage, read_write> destination: array<u32>;

fn sample_at(x: i32, y: i32) -> u32 {
    let column = u32(clamp(x, 0, i32(image.width) - 1));
    let row = u32(clamp(y, 0, i32(image.height) - 1));
    return source[row * image.width + column];
}

fn weighted_sum(x: i32, y: i32, dx: i32, dy: i32) -> u32 {
    return sample_at(x - 2 * dx, y - 2 * dy)
        + 4u * sample_at(x - dx, y - dy)
        + 6u * sample_at(x, y)
        + 4u * sample_at(x + dx, y + dy)
        + sample_at(x + 2 * dx, y + 2 * dy);
}

@compute @workgroup_size(16, 16)
fn horizontal(@builtin(global_invocation_id) pixel: vec3<u32>) {
    if (pixel.x < image.width && pixel.y < image.height) {
        destination[pixel.y * image.width + pixel.x] =
            weighted_sum(i32(pixel.x), i32(pixel.y), 1, 0);
    }
}

@compute @workgroup_size(16, 16)
fn vertical(@builtin(global_invocation_id) pixel: vec3<u32>) {
    if (pixel.x < image.width && pixel.y < image.height) {
        destination[pixel.y * image.width + pixel.x] =
            (weighted_sum(i32(pixel.x), i32(pixel.y), 0, 1) + 128u) / 256u;
    }
}
