
macro_rules! offset_of {
    ($type:ty: $($name:ident).+) => {
        unsafe {
            let value: $type = ::std::mem::uninitialized();
            let offset = &value $(.$name)+ as *const _ as usize;
            let base = &value as *const _ as usize;
            offset - base
        }
    }
}

#[macro_use]
extern crate glsl_layout;
use glsl_layout::*;
fn main() {
    use std::mem::size_of;

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Uniform)]
    struct Foo {
        x: int,
        y: vec3,
        z: float,
        w: mat4x4,
        a: [f32; 3],
    }

    type UFoo = <Foo as Uniform>::Std140;

    fn align_to(offset: usize, align: usize) -> usize {
        if offset % align == 0 {
            offset
        } else {
            (((offset - 1) / align) + 1) * align
        }
    }

    assert_eq!(
        offset_of!(UFoo: y),
        align_to(size_of::<int>(), 16),
    );

    assert_eq!(
        offset_of!(UFoo: z),
        align_to(offset_of!(UFoo: y) + size_of::<vec3>(), 4),
    );

    let _ = Foo {
        x: 2,
        y: [0.0; 3].into(),
        z: 0.0,
        w: [[0.0; 4]; 4].into(),
        a: [0.0; 3].into(),
    }.std140();
}
