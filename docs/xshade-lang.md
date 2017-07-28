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

## operators
### unary
#### Syntax

```rust
let right  : U
let result : V /* V is either T or U */

result = <op> right
``` 

| symbol | description | explanation | example | 
|--------|-------------|-------------|---------|
| `!` | logical negation | Negates an expression `x`, if it is implicitly convertible to a boolean value. | `let y = !x;` |
| `-` | negation | Negates a numeric value of non-boolean primitive type. | `let y = -x;` |
Remarks: None
### binary
#### Syntax

```rust
let left   : T
let right  : U
let result : V /* V is either T or U */

result = left <op> right
```

| symbol | description | explanation | example | 
|--------|-------------|-------------|---------|
| `=` | assignment | Assigns a value of type `U` to a constant/variable of type `T`, if `U` is convertible to `T`. | `let x = 0; ` |
| `+` | addition | Adds a value of type `U` with another value of type `T`.  | `let y = x + 1;` |
| `-` | subtraction | Subtracts a value of type `U` from a value of type `T`.  | `let y = x - 1;` |
| `*` | multiplication | Multiplies a value of type `T` with a value of type `U`. | `let y = x * 2;` |
| `/` | division | Divides a value of type `T` by a value of type `U`. |  `let y = x / 2;` |


**Remarks**:  For each of the above operators the result type is evaluated according to the implicit type conversion rules. 


[TODO: ADD CHAPTER REF TO TYPE IMPLICIT CONVERSIONS]
### binary-assign

#### Syntax

```rust 
let left   : T
let right  : U

left <op> right
```

| symbol | description | example |
|--------|-------------|---------|
| `+=` | addition-assignment | `x += 1;` |
| `-=` | subtraction-assignment | `x -= 1;` |
| `*=` | multiplication-assignment | `x *= 2;` |
| `/=` | division-assignment | `x /= 2;` |

#### Remarks
If `U` is a higher-priorized type than `T`, the assignment is invalid due to truncation and possible data loss.
E.g.:

```rust
let left  : i32 = 2
let right : f32 = 1.445f

left += right // Error: f32 will be truncated to i32. Data loss.
```

### comparison
| symbol | description | example |
|--------|-------------|---------|
| `==` | equal | `let eq = x == y;` |
| `!=` | not equal | `x -= 1;` |
| `<` | less | `x *= 2;` |
| `<=` | less-equal | `x /= 2;` |
| `>` | greater | `x /= 2;` |
| `>=` | greater-equal | `x /= 2;` |

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