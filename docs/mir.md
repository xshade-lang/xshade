# MIR - mid-level intermediate representation

* mir is generated per executable unit of code, e.g. for single shaders and kernels.
* structured like a control flow graph.
* has no functions or calls, everything gets inlined to generate mir.

## Blocks

### Entry
The entry into a mir block
```rust
pub struct EntryBlock {
    pub address: usize,
    pub branch_next: Link,
}
```

### If
```rust
pub struct IfBlock {
    pub address: usize,
    pub branch_then: Link,
    pub branches_else: Vec<Link>,
    pub branch_next: Link,
}
```

### Constants
```rust
pub struct Constant {
    pub address: usize,
    pub value: ConstantValue,
}

pub enum ConstantValue {
    I32(i32),
    U32(u32),
    // ...
}
```

## Links
```rust
pub enum Link {
    Address(usize),
    Void,
}
```
