# xshade language

## Type-System
### Primitive Types
| name | description |
|------|-------------|
| bool | boolean value |
| i32  | 32 bit signed integer |
| i64  | 64 bit signed integer |
| u32  | 64 bit unsigned integer |
| u64  | 64 bit unsigned integer |
| f32  | 32 bit floating point number |
| f64  | 64 bit floating point number |

### Primitive Type Literals
| name | literal            | range-min | range-max |
|------|:--------------------|-----------|-----------|
| bool | `true` or `false`  | `N/A`
| i32  | `-123` or `0` or `123` | `-2.147.483.648` | `2.147.483.647` |
| i64  | `-123` or `0` or `123` | `-9.223.372.036.854.775.808` | `9.223.372.036.854.775.807` |
| u32  | `0` or `123` | `0` | `4.294.967.295` |
| u64  | `0` or `123` | `0` | `18.446.744.073.709.551.615` |
| f32  | `-1.01f` or `1.234f` | `1.175494e-38` | `3.402823e+38` |
| f64  | `-1.01` or `1.234` | `2.225074e-308` | `1.797693e+308` |

### Templated & Complex Types
| name | description |
|------|-------------|
| vec<T, N>    | Vector of type `T` with `N` components. |
| mat<T, N, M> | Matrix of type `T` with `N` components in dimension-0 and `M` components in dimension-1. |
| list<T, N>   | List containing `N` instances of user-defined type `T`. |

#### Constraints
Vector and Matrix types must have at least 2 components per dimension, i.e. `N, M element [2 .. 4]`.

#### Examples
```rust
let x: vec<f32, 3>;    // 3 component f32 vector.
let y: mat<f64, 4, 4>; // 16-component square f64 matrix.

type vec3 = vec<f32, 3>;
type vec4 = vec<f32, 4>;

struct Color {
    r : f32;
    g : f32;
    b : f32;
    a : f32;
};

struct Light { 
    position:  vec3;
    direction: vec3;
    color:     vec4;
};

let z: list<Light, 32>;
```

#### Aliases
| alias      | resolved to      | alias      | resolved to      |
|------------|------------------| -----------|------------------|
| _templated_  |                  | _default_    |                  |
| vec2\<T>   | `vec<T, 2>`      | vec2       | `vec<f32, 2>`    |
| vec3\<T>   | `vec<T, 3>`      | vec3       | `vec<f32, 3>`    |
| vec4\<T>   | `vec<T, 4>`      | vec4       | `vec<f32, 4>`    |
| mat2x2\<T> | `mat<T, 2, 2>`   | mat2x2     | `mat<f32, 2, 2>` |
| mat2x3\<T> | `mat<T, 2, 3>`   | mat2x3     | `mat<f32, 2, 3>` |
| mat2x4\<T> | `mat<T, 2, 4>`   | mat2x4     | `mat<f32, 2, 4>` |
| mat3x2\<T> | `mat<T, 3, 2>`   | mat3x4     | `mat<f32, 3, 2>` |
| mat3x3\<T> | `mat<T, 3, 3>`   | mat3x4     | `mat<f32, 3, 3>` |
| mat3x4\<T> | `mat<T, 3, 4>`   | mat3x4     | `mat<f32, 3, 4>` |
| mat4x2\<T> | `mat<T, 4, 2>`   | mat4x2     | `mat<f32, 4, 2>` |
| mat4x3\<T> | `mat<T, 4, 3>`   | mat4x3     | `mat<f32, 4, 3>` |
| mat4x4\<T> | `mat<T, 4, 4>`   | mat4x4     | `mat<f32, 4, 4>` |

### User-defined Types
### Special Types
#### Sampler

### Type-Declaration
#### Variables
##### Syntax
```rust
let <name>: <type>; // Declaration, or
let <name>;         // also declaration; type-suffix is optional! 
<name> = <value>;   // Definition

// Declaration and definition at once.
let <name> [:<type>] = <value>;   
```
##### Brief
Variables are mutable placeholders for arbitary values of a type `T`. <br />
Variables can be assigned to in regular fashion by `<name> = <var:T>/<literal:T>;` 
<br />

##### Detail
As given in the example above, variables are declared by `let` followed by it's `<name>`, succeeded by a type-declarator `: <type>`. <br />
Explicit denotion of the type is optional, since it will otherwise be derived by the compiler.
Both styles are legit. 

Variables can be assigned to in regular fashion by `<name> = <value>;` or immediately when declarating the variable by appending `= <value>;` after the name or type-declarator, if used.

Implicit type-conversion is only supported for `i32 -> i64`. <br />
(See `Type-Conversion` for more information.)

#### Constants
##### Syntax

```rust 
// Constant definition, must be initialized immediately.
const <name> [:<type>] = <value>; 
```

#### Type-Conversion

----

## Operators
### Unary
#### Syntax

```rust
let right: U = <value>;

let result_from_var: U  = <op>right;
let result_from_expr: U = <op>(<expr: U>)
``` 

| symbol | description | explanation | examples | 
|--------|-------------|-------------|----------|
| `!` | logical negation | Negates an expression `x`. if it is implicitly convertible to a boolean value. | `let x: bool = true;` <br /> `let y: bool = !x;` <br /> `let z: i32 = 127;` <br /> `let w: bool = !(z>128);` |
| `-` | negation | Negates a numeric value of non-boolean primitive type. | `let x: i8 = -127;` <br /> `let y: i8 = (-x + 1);` |
#### Remarks:
The `logical negation` will only apply to boolean expressions or values. <br />
The `negation` will only apply to non-boolean expressions or values. 

----

### Binary
#### Syntax

```rust
let left: T  = T{};
let right: U = U{};

let result = (left <op> right); /* V is either T or U */
```

| symbol | description | explanation | example | 
|--------|-------------|-------------|---------|
| `=` | assignment | Assigns a value of type `U` to a constant/variable of type `T`, if `U` is convertible to `T`. | `let x = 0; ` |
| `+` | addition | Adds a value of type `U` with another value of type `T`.  | `let y = x + 1;` |
| `-` | subtraction | Subtracts a value of type `U` from a value of type `T`.  | `let y = x - 1;` |
| `*` | multiplication | Multiplies a value of type `T` with a value of type `U`. | `let y = x * 2;` |
| `/` | division | Divides a value of type `T` by a value of type `U`. |  `let y = x / 2;` |


#### Remarks:  
For each of the above operators the result type is evaluated according to the implicit type conversion rules. 

[TODO: ADD CHAPTER REF TO TYPE IMPLICIT CONVERSIONS]

----

### Binary-Assign

#### Syntax

```rust 
let left: T  = T{};
let right: U = U{};

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
let left: i32  = 2;
let right: f32 = 1.445f

left += right // Error: f32 will be truncated to i32. Data loss.
```

----

### Comparison
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