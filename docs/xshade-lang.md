# xshade language

## primitive types
| name | description |
|------|-------------|
| bool | boolean value |
| i32  | 32 bit signed integer |
| i64  | 64 bit signed integer |
| u32  | 64 bit unsigned integer |
| u64  | 64 bit unsigned integer |
| f32  | 32 bit floating point number |
| f64  | 64 bit floating point number |

## constant declarations
```xshade
const myConstant: mat4x4;
```

## sampler declarations
```xshade
sampler mySampler: Sampler2d;
```

## structure declarations
```xshade
struct MyStruct {
    myMember: f32,
}
```

## function declarations
```xshade
fn main(input: MyStruct) -> MyOtherStruct {
    // function body
}
```

## program declarations
```xshade
program MyProgram {
    vertex: myVertexFunction,
    fragment: myFragmentFunction,
}
```

## example program
```xshade
sampler albedo: Sampler2d;
const mvp: Matrix4x4;

struct VertexInput {
    [bind(Position, 0)]
    position: vec3,
    [bind(TextureCoordinates, 0)]
    uv: vec2,
}

struct VertexOutput {
    [bind(Position, 0)]
    position: vec4,
    [bind(TextureCoordinates, 0)]
    uv: vec2,
}

struct FragmentInput {
    [bind(TextureCoordinates, 0)]
    uv: vec2,
}

fn vertexShader(intput: VertexInput) -> VertexOutput {
    VertexOutput {
        position: mvp * vec4(input.position, 1.0)
    }
}

fn fragmentShader(intput: FragmentInput) -> vec4 {
    albedo(input.uv)
}

program Diffuse {
    vertex: vertexShader,
    fragment: fragmentShader,
}
```