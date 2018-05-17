
# GLSL-LAYOUT

This crates provides data types and traits to build structures ready to upload into UBO.

## Usage

Deriving `Uniform` with proc-macro will produce associated type `Std140`
with same data layout as uniform blocks declared with `layout(std140)`.
All members of structure that derives `Uniform` must implement `Uniform` as well.

Implementing `Uniform` should be done via deriving.
Implementing it manually is possible.
It requires to provide associated type `Std140` which must implement `Std140` trait.
But trait `Std140` is marked unsafe so be careful.

Trait `Uniform` also requires `Copy`.

Typical usage scenario is:
```rust
#[derive(Copy, Clone, Uniform)]
struct FragmentArgs {
    pos: vec3,
    dir: vec3,
    count: uint,
}

write_to_buffer(&FragmentArgs {
    pos: [0.0, 1.0, 2.0].into(),
    dir: [3.0, 4.0, 5.0].into(),
    count: 42,
}.std140());

```

### Data types

There are basic data types from glsl:
* boolean (name `bool` is already occupied)
* int
* uint
* float
* double

Also more complex types:
* vectors   - (vec2, vec3, vec4, bvec2, ivec2, uvec2, dvec2 etc)
* matrices  - (mat2x3, dmat4 etc)
* arrays    - 

## License

`glsl-layout` is free and open source software distributed under the terms of both
the [MIT License][lm] and the [Apache License 2.0][la].

[lm]: LICENSE-MIT
[la]: LICENSE-APACHE

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Known issues

### On MacOS
If uniform block contains array of structures and this array accessed with dynamic index (variable instead of literal)
it may load wrong bytes for members not aligned to size of `vec4` (16 bytes). In this case manual padding can be the fix.
