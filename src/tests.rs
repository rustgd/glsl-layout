
use scalar::*;
use vec::*;
use mat::*;

#[test]
fn rule1() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<boolean>(), size_of::<boolean>());
    assert_eq!(align_of::<int>(), size_of::<int>());
    assert_eq!(align_of::<uint>(), size_of::<uint>());
    assert_eq!(align_of::<float>(), size_of::<float>());
    assert_eq!(align_of::<double>(), size_of::<double>());
}

#[test]
fn rule2() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<bvec2>(), size_of::<bvec2>());
    assert_eq!(align_of::<ivec2>(), size_of::<ivec2>());
    assert_eq!(align_of::<uvec2>(), size_of::<uvec2>());
    assert_eq!(align_of::< vec2>(), size_of::< vec2>());
    assert_eq!(align_of::<dvec2>(), size_of::<dvec2>());

    assert_eq!(align_of::<bvec4>(), size_of::<bvec4>());
    assert_eq!(align_of::<ivec4>(), size_of::<ivec4>());
    assert_eq!(align_of::<uvec4>(), size_of::<uvec4>());
    assert_eq!(align_of::< vec4>(), size_of::< vec4>());
    assert_eq!(align_of::<dvec4>(), size_of::<dvec4>());
}


#[test]
fn rule3() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<bvec3>(), size_of::<bvec4>());
    assert_eq!(align_of::<ivec3>(), size_of::<ivec4>());
    assert_eq!(align_of::<uvec3>(), size_of::<uvec4>());
    assert_eq!(align_of::< vec3>(), size_of::< vec4>());
    assert_eq!(align_of::<dvec3>(), size_of::<dvec4>());
}

#[test]
fn rule4() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<array![boolean; 3]>(), size_of::< vec4>());
    assert_eq!(align_of::<array![int; 1]>(), size_of::< vec4>());
    assert_eq!(align_of::<array![float; 7]>(), size_of::< vec4>());
    assert_eq!(align_of::<array![dvec4; 2]>(), size_of::<dvec4>());
}

#[test]
fn rule5() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<bmat3>(), size_of::< vec4>());
    assert_eq!(align_of::<imat2>(), size_of::< vec4>());
    assert_eq!(align_of::<mat4>(), size_of::< vec4>());
    assert_eq!(align_of::<dmat3x2>(), size_of::<dvec2>());
    assert_eq!(align_of::<dmat2x3>(), size_of::<dvec4>());
}

#[test]
fn test_struct() {
    pub use std::mem::{align_of, size_of};

    uniform!(struct Foo {
        x: i32,
    });

    uniform!(struct Bar {
        x: dvec3,
    });

    assert_eq!(align_of::<Foo>(), size_of::< vec4>());
    assert_eq!(align_of::<Bar>(), size_of::<dvec4>());
}
