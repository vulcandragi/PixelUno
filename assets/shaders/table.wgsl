#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> size: vec2f;

// Correção do gama. Linear = srgb^2.2
fn srgb_to_linear(srgb: vec3f) -> vec4f {
    return vec4f(pow(srgb, vec3f(2.2)), 1.0);
}

fn border_range(current_pixel: vec2f, range: vec2f) -> bool {
    let in_range = (current_pixel.x > (size.x - range.x))
        || (current_pixel.x < range.x)
        || (current_pixel.y > (size.y - range.x))
        || (current_pixel.y < range.x);

    return in_range
        && current_pixel.x < (size.x - range.y)
        && current_pixel.x > range.y
        && current_pixel.y < (size.y - range.y)
        && current_pixel.y > range.y;
}

fn border_radius(current_pixel: vec2f, range: vec2f, curve: i32) -> bool {
    return true;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let current_pixel = size * mesh.uv;

    if (border_range(current_pixel, vec2f(90, 85))) {
        // #3d0a12
        let stroke_color = srgb_to_linear(vec3f(0.24, 0.04, 0.07));
        return stroke_color;
    } else {
        // #7d1523
        let center_color = srgb_to_linear(vec3f(0.490, 0.082, 0.137));
        // #5c101c
        let border_color = srgb_to_linear(vec3f(0.3607, 0.0627, 0.1098));
    
        let center = vec2f(0.5, 0.5);
        let distance = length((mesh.uv - center) * vec2f(size.x / size.y, 1.0));
    
        let mixin = clamp(distance * 2, 0, 1);
    
        return mix(center_color, border_color, mixin);
    }

    return vec4f(0, 0, 0, 0);
}
