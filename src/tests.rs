
use scalar::*;

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

    assert_eq!(align_of::<bvec![2]>(), size_of::<bvec![2]>());
    assert_eq!(align_of::<ivec![2]>(), size_of::<ivec![2]>());
    assert_eq!(align_of::<uvec![2]>(), size_of::<uvec![2]>());
    assert_eq!(align_of::<fvec![2]>(), size_of::<fvec![2]>());
    assert_eq!(align_of::<dvec![2]>(), size_of::<dvec![2]>());

    assert_eq!(align_of::<bvec![4]>(), size_of::<bvec![4]>());
    assert_eq!(align_of::<ivec![4]>(), size_of::<ivec![4]>());
    assert_eq!(align_of::<uvec![4]>(), size_of::<uvec![4]>());
    assert_eq!(align_of::<fvec![4]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<dvec![4]>(), size_of::<dvec![4]>());
}


#[test]
fn rule3() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<bvec![3]>(), size_of::<bvec![4]>());
    assert_eq!(align_of::<ivec![3]>(), size_of::<ivec![4]>());
    assert_eq!(align_of::<uvec![3]>(), size_of::<uvec![4]>());
    assert_eq!(align_of::<fvec![3]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<dvec![3]>(), size_of::<dvec![4]>());
}

#[test]
fn rule4() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<array![boolean; 3]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<array![int; 1]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<array![float; 7]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<array![dvec![4]; 2]>(), size_of::<dvec![4]>());
}

#[test]
fn rule5() {
    pub use std::mem::{align_of, size_of};

    assert_eq!(align_of::<matrix![boolean; 3 * 3]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<matrix![int; 1 * 2]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<matrix![float; 4 * 4]>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<matrix![double; 2 * 2]>(), size_of::<dvec![2]>());
    assert_eq!(align_of::<matrix![double; 2 * 3]>(), size_of::<dvec![4]>());
}

#[test]
fn test_struct() {
    pub use std::mem::{align_of, size_of};

    uniform!(struct Foo {
        x: i32,
    });

    uniform!(struct Bar {
        x: dvec![3],
    });

    assert_eq!(align_of::<Foo>(), size_of::<fvec![4]>());
    assert_eq!(align_of::<Bar>(), size_of::<dvec![4]>());
}
