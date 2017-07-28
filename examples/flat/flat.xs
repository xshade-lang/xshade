const mvp: mat4x4;
const flatColor: vec4;

struct VertexInput {
    position: vec3,
}

struct VertexOutput {
    position: vec4,
}

fn vertexShader(intput: VertexInput) -> VertexOutput {
    let output = VertexOutput {
        position: mvp * vec4(input.position, 1.0),
    };

    return output;
}

fn fragmentShader() -> vec4 {
    let color = flatColor;
    return color;
}

program Flat {
    vertex: vertexShader,
    fragment: fragmentShader,
}
