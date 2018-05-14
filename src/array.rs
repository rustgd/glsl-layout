
use std::{
    marker::PhantomData,
    slice::{Iter as SliceIter, IterMut as SliceIterMut},
};

use align::Align16;
use uniform::{Std140, Uniform};

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
    T: Uniform
{
    fn from(values: T) -> Self {
        Element(values, T::align())
    }
}

impl<T> AsRef<T> for Element<T>
where
    T: Uniform
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Element<T>
where
    T: Uniform
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
    pub fn iter<'a>(&'a self) -> ArrayIter<SliceIter<'a, Element<T>>> {
        ArrayIter(self.0.as_ref().iter())
    }

    pub fn iter_mut<'a,>(&'a mut self) -> ArrayIter<SliceIterMut<'a, Element<T>>> {
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

macro_rules! impl_array {
    ($size:tt) => {
        impl<T, U, F> MapArray<[T; $size], F> for [U; $size]
        where
            F: FnMut(T) -> U,
        {
            fn map_array(mut values: [T; $size], mut f: F) -> Self {
                use std::{mem::forget, ptr::{read, write}};

                unsafe {
                    // All elements of `result` is written.
                    // Each element of `values` read once and then forgotten.
                    // Hence safe in case `f` never panics.
                    // TODO: Make it panic-safe.
                    let mut result: [U; $size] = ::std::mem::uninitialized();
                    for i in 0 .. $size {
                        write(&mut result[i], f(read(&mut values[i])));
                    }
                    forget(values);
                    result
                }
            }
        }

        impl<T, U> From<[T; $size]> for Array<U, [U; $size]>
        where
            T: Into<U>,
        {
            fn from(values: [T; $size]) -> Self {
                Array(MapArray::map_array(values, T::into), PhantomData)
            }
        }

        impl<T, U> From<[T; $size]> for Array<U, [Element<U>; $size]>
        where
            T: Into<U>,
            U: Uniform,
        {
            fn from(values: [T; $size]) -> Self {
                let values: [U; $size] = MapArray::map_array(values, T::into);
                Array(MapArray::map_array(values, U::into), PhantomData)
            }
        }

        impl<T> Uniform for [T; $size]
        where
            T: Uniform,
        {
            type Align = Align16;
            type Std140 = Array<T::Std140, [Element<T::Std140>; $size]>;

            fn align() -> Align16 { Align16 }
            fn std140(&self) -> Array<T::Std140, [Element<T::Std140>; $size]> {
                use std::ptr::write;
                unsafe {
                    // All elements of `result` is written.
                    let mut result: [Element<T::Std140>; $size] = ::std::mem::uninitialized();
                    for i in 0 .. $size {
                        write(&mut result[i], self[i].std140().into());
                    }
                    Array(result, PhantomData)
                }
            }
        }

        impl<T> Uniform for Array<T, [Element<T>; $size]>
        where
            T: Uniform,
        {
            type Align = Align16;
            type Std140 = Array<T::Std140, [Element<T::Std140>; $size]>;

            fn align() -> Align16 { Align16 }
            fn std140(&self) -> Array<T::Std140, [Element<T::Std140>; $size]> {
                use std::ptr::write;
                unsafe {
                    // All elements of `result` is written.
                    let mut result: [Element<T::Std140>; $size] = ::std::mem::uninitialized();
                    for i in 0 .. $size {
                        write(&mut result[i], self.0[i].0.std140().into());
                    }
                    Array(result, PhantomData)
                }
            }
        }

        unsafe impl<T> Std140 for Array<T, [Element<T>; $size]>
        where
            T: Std140,
        {}
    }
}

impl_array!(000);
impl_array!(001);
impl_array!(002);
impl_array!(003);
impl_array!(004);
// impl_array!(005);
// impl_array!(006);
// impl_array!(007);
// impl_array!(008);
// impl_array!(009);
// impl_array!(010);
// impl_array!(011);
// impl_array!(012);
// impl_array!(013);
// impl_array!(014);
// impl_array!(015);
// impl_array!(016);
// impl_array!(017);
// impl_array!(018);
// impl_array!(019);
// impl_array!(020);
// impl_array!(021);
// impl_array!(022);
// impl_array!(023);
// impl_array!(024);
// impl_array!(025);
// impl_array!(026);
// impl_array!(027);
// impl_array!(028);
// impl_array!(029);
// impl_array!(030);
// impl_array!(031);
// impl_array!(032);

#[cfg(feature="bigger-arrays")]
mod impl_bigger_arrays {
use super::*;
impl_array!(033);
impl_array!(034);
impl_array!(035);
impl_array!(036);
impl_array!(037);
impl_array!(038);
impl_array!(039);
impl_array!(040);
impl_array!(041);
impl_array!(042);
impl_array!(043);
impl_array!(044);
impl_array!(045);
impl_array!(046);
impl_array!(047);
impl_array!(048);
impl_array!(049);
impl_array!(050);
impl_array!(051);
impl_array!(052);
impl_array!(053);
impl_array!(054);
impl_array!(055);
impl_array!(056);
impl_array!(057);
impl_array!(058);
impl_array!(059);
impl_array!(060);
impl_array!(061);
impl_array!(062);
impl_array!(063);
impl_array!(064);
impl_array!(065);
impl_array!(066);
impl_array!(067);
impl_array!(068);
impl_array!(069);
impl_array!(070);
impl_array!(071);
impl_array!(072);
impl_array!(073);
impl_array!(074);
impl_array!(075);
impl_array!(076);
impl_array!(077);
impl_array!(078);
impl_array!(079);
impl_array!(080);
impl_array!(081);
impl_array!(082);
impl_array!(083);
impl_array!(084);
impl_array!(085);
impl_array!(086);
impl_array!(087);
impl_array!(088);
impl_array!(089);
impl_array!(090);
impl_array!(091);
impl_array!(092);
impl_array!(093);
impl_array!(094);
impl_array!(095);
impl_array!(096);
impl_array!(097);
impl_array!(098);
impl_array!(099);
impl_array!(100);
impl_array!(101);
impl_array!(102);
impl_array!(103);
impl_array!(104);
impl_array!(105);
impl_array!(106);
impl_array!(107);
impl_array!(108);
impl_array!(109);
impl_array!(110);
impl_array!(111);
impl_array!(112);
impl_array!(113);
impl_array!(114);
impl_array!(115);
impl_array!(116);
impl_array!(117);
impl_array!(118);
impl_array!(119);
impl_array!(120);
impl_array!(121);
impl_array!(122);
impl_array!(123);
impl_array!(124);
impl_array!(125);
impl_array!(126);
impl_array!(127);
impl_array!(128);
impl_array!(129);
impl_array!(130);
impl_array!(131);
impl_array!(132);
impl_array!(133);
impl_array!(134);
impl_array!(135);
impl_array!(136);
impl_array!(137);
impl_array!(138);
impl_array!(139);
impl_array!(140);
impl_array!(141);
impl_array!(142);
impl_array!(143);
impl_array!(144);
impl_array!(145);
impl_array!(146);
impl_array!(147);
impl_array!(148);
impl_array!(149);
impl_array!(150);
impl_array!(151);
impl_array!(152);
impl_array!(153);
impl_array!(154);
impl_array!(155);
impl_array!(156);
impl_array!(157);
impl_array!(158);
impl_array!(159);
impl_array!(160);
impl_array!(161);
impl_array!(162);
impl_array!(163);
impl_array!(164);
impl_array!(165);
impl_array!(166);
impl_array!(167);
impl_array!(168);
impl_array!(169);
impl_array!(170);
impl_array!(171);
impl_array!(172);
impl_array!(173);
impl_array!(174);
impl_array!(175);
impl_array!(176);
impl_array!(177);
impl_array!(178);
impl_array!(179);
impl_array!(180);
impl_array!(181);
impl_array!(182);
impl_array!(183);
impl_array!(184);
impl_array!(185);
impl_array!(186);
impl_array!(187);
impl_array!(188);
impl_array!(189);
impl_array!(190);
impl_array!(191);
impl_array!(192);
impl_array!(193);
impl_array!(194);
impl_array!(195);
impl_array!(196);
impl_array!(197);
impl_array!(198);
impl_array!(199);
impl_array!(200);
impl_array!(201);
impl_array!(202);
impl_array!(203);
impl_array!(204);
impl_array!(205);
impl_array!(206);
impl_array!(207);
impl_array!(208);
impl_array!(209);
impl_array!(210);
impl_array!(211);
impl_array!(212);
impl_array!(213);
impl_array!(214);
impl_array!(215);
impl_array!(216);
impl_array!(217);
impl_array!(218);
impl_array!(219);
impl_array!(220);
impl_array!(221);
impl_array!(222);
impl_array!(223);
impl_array!(224);
impl_array!(225);
impl_array!(226);
impl_array!(227);
impl_array!(228);
impl_array!(229);
impl_array!(230);
impl_array!(231);
impl_array!(232);
impl_array!(233);
impl_array!(234);
impl_array!(235);
impl_array!(236);
impl_array!(237);
impl_array!(238);
impl_array!(239);
impl_array!(240);
impl_array!(241);
impl_array!(242);
impl_array!(243);
impl_array!(244);
impl_array!(245);
impl_array!(246);
impl_array!(247);
impl_array!(248);
impl_array!(249);
impl_array!(250);
impl_array!(251);
impl_array!(252);
impl_array!(253);
impl_array!(254);
impl_array!(255);
impl_array!(256);
}
