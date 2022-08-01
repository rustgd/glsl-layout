use crate::align::Align16;
use crate::uniform::{Std140, Uniform};
use std::{
    marker::PhantomData,
    slice::{Iter as SliceIter, IterMut as SliceIterMut},
};

pub(crate) trait MapArray<A, F> {
    fn map_array(values: A, f: F) -> Self;
}

/// Aligning wrapper.
/// Elements for array are aligned to 16 bytes (size of vec4) at least.
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[repr(C, align(16))]
pub struct Element<T: Uniform>(pub T, pub T::Align);

impl<T> From<T> for Element<T>
where
    T: Uniform,
{
    fn from(values: T) -> Self {
        Element(values, Default::default())
    }
}

impl<T> AsRef<T> for Element<T>
where
    T: Uniform,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Element<T>
where
    T: Uniform,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Array of `Element`s.
/// This type implements useful traits for converting from unwrapped types.
#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[repr(C, align(16))]
pub struct Array<T, A>(pub A, pub PhantomData<fn(T)>);

impl<T, A> Array<T, A> {
    pub fn new(array: A) -> Self {
        Array(array, PhantomData)
    }
}

impl<T, A> AsRef<A> for Array<T, A> {
    fn as_ref(&self) -> &A {
        &self.0
    }
}

impl<T, A> AsMut<A> for Array<T, A> {
    fn as_mut(&mut self) -> &mut A {
        &mut self.0
    }
}

impl<T, A> Array<T, A>
where
    T: Uniform,
    A: AsMut<[Element<T>]> + AsRef<[Element<T>]>,
{
    pub fn iter(&self) -> ArrayIter<SliceIter<Element<T>>> {
        ArrayIter(self.0.as_ref().iter())
    }

    pub fn iter_mut(&mut self) -> ArrayIter<SliceIterMut<Element<T>>> {
        ArrayIter(self.0.as_mut().iter_mut())
    }
}

impl<'a, T, A> IntoIterator for &'a Array<T, A>
where
    T: Uniform,
    A: AsMut<[Element<T>]> + AsRef<[Element<T>]>,
{
    type Item = &'a T;
    type IntoIter = ArrayIter<SliceIter<'a, Element<T>>>;

    fn into_iter(self) -> ArrayIter<SliceIter<'a, Element<T>>> {
        self.iter()
    }
}

impl<'a, T, A> IntoIterator for &'a mut Array<T, A>
where
    T: Uniform,
    A: AsMut<[Element<T>]> + AsRef<[Element<T>]>,
{
    type Item = &'a mut T;
    type IntoIter = ArrayIter<SliceIterMut<'a, Element<T>>>;

    fn into_iter(self) -> ArrayIter<SliceIterMut<'a, Element<T>>> {
        self.iter_mut()
    }
}

/// Array ref iterator
/// Iterate over references to inner values.
pub struct ArrayIter<I>(I);

impl<'a, T> Iterator for ArrayIter<SliceIter<'a, Element<T>>>
where
    T: Uniform,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.0.next().map(|elem| &elem.0)
    }
}

impl<'a, T> ExactSizeIterator for ArrayIter<SliceIter<'a, Element<T>>>
where
    T: Uniform,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> DoubleEndedIterator for ArrayIter<SliceIter<'a, Element<T>>>
where
    T: Uniform,
{
    fn next_back(&mut self) -> Option<&'a T> {
        self.0.next_back().map(|elem| &elem.0)
    }
}

impl<'a, T> Iterator for ArrayIter<SliceIterMut<'a, Element<T>>>
where
    T: Uniform,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.0.next().map(|elem| &mut elem.0)
    }
}

impl<'a, T> ExactSizeIterator for ArrayIter<SliceIterMut<'a, Element<T>>>
where
    T: Uniform,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> DoubleEndedIterator for ArrayIter<SliceIterMut<'a, Element<T>>>
where
    T: Uniform,
{
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.0.next_back().map(|elem| &mut elem.0)
    }
}

impl<T, U, F, const N: usize> MapArray<[T; N], F> for [U; N]
where
    F: FnMut(T) -> U,
{
    fn map_array(values: [T; N], mut f: F) -> Self {
        use std::{
            mem::{ManuallyDrop, MaybeUninit},
            ptr::{read, write},
        };

        // Use `ManuallyDrop<_>` to guard against panic safety issue.
        // Upon panic in `f`, `values` isn't dropped
        // and thus item copied by `read()` is dropped only once.
        let mut values = ManuallyDrop::new(values);
        unsafe {
            let mut result: MaybeUninit<[U; N]> = MaybeUninit::zeroed();
            for i in 0..N {
                write(
                    result.as_mut_ptr().cast::<U>().add(i),
                    f(read(&mut values[i])),
                );
            }
            result.assume_init()
        }
    }
}

impl<T, U, const N: usize> From<[T; N]> for Array<U, [U; N]>
where
    T: Into<U>,
{
    fn from(values: [T; N]) -> Self {
        Array(MapArray::map_array(values, T::into), PhantomData)
    }
}

impl<T, U, const N: usize> From<[T; N]> for Array<U, [Element<U>; N]>
where
    T: Into<U>,
    U: Uniform,
{
    fn from(values: [T; N]) -> Self {
        let values: [U; N] = MapArray::map_array(values, T::into);
        Array(MapArray::map_array(values, U::into), PhantomData)
    }
}

impl<T, const N: usize> Uniform for [T; N]
where
    T: Uniform,
{
    type Align = Align16;
    type Std140 = Array<T::Std140, [Element<T::Std140>; N]>;

    fn std140(&self) -> Array<T::Std140, [Element<T::Std140>; N]> {
        use std::ptr::write;
        unsafe {
            // All elements of `result` is written.
            let mut result: ::std::mem::MaybeUninit<[Element<T::Std140>; N]> =
                ::std::mem::MaybeUninit::zeroed();
            for i in 0..N {
                write(
                    result.as_mut_ptr().cast::<Element<T::Std140>>().add(i),
                    self[i].std140().into(),
                );
            }
            Array(result.assume_init(), PhantomData)
        }
    }
}

impl<T, const N: usize> Uniform for Array<T, [Element<T>; N]>
where
    T: Uniform,
{
    type Align = Align16;
    type Std140 = Array<T::Std140, [Element<T::Std140>; N]>;

    fn std140(&self) -> Array<T::Std140, [Element<T::Std140>; N]> {
        use std::ptr::write;
        unsafe {
            // All elements of `result` is written.
            let mut result: ::std::mem::MaybeUninit<[Element<T::Std140>; N]> =
                ::std::mem::MaybeUninit::zeroed();
            for i in 0..N {
                write(
                    result.as_mut_ptr().cast::<Element<T::Std140>>().add(i),
                    self.0[i].0.std140().into(),
                );
            }
            Array(result.assume_init(), PhantomData)
        }
    }
}

unsafe impl<T, const N: usize> Std140 for Array<T, [Element<T>; N]> where T: Std140 {}

#[test]
fn test_array() {
    use crate::{mat4, vec2, vec3};

    let _ = [vec2::default(), vec2::default()].std140();
    let _ = [
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
        vec3::default(),
    ]
    .std140();

    let _ = [
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
        mat4::default(),
    ]
    .std140();
}
