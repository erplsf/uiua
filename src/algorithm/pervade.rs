//! Algorithms for pervasive array operations

use std::{
    cmp::{self, Ordering},
    convert::Infallible,
    fmt::Display,
    marker::PhantomData,
    slice::{self, Chunks},
};

use crate::{array::*, Uiua, UiuaError, UiuaResult};

use super::max_shape;

#[allow(clippy::len_without_is_empty)]
pub trait Arrayish {
    type Value: ArrayValue;
    fn shape(&self) -> &[usize];
    fn data(&self) -> &[Self::Value];
    fn rank(&self) -> usize {
        self.shape().len()
    }
    fn flat_len(&self) -> usize {
        self.data().len()
    }
    fn row_len(&self) -> usize {
        self.shape().iter().skip(1).product()
    }
    fn rows(&self) -> Chunks<Self::Value> {
        self.data().chunks(self.row_len().max(1))
    }
    fn shape_prefixes_match(&self, other: &impl Arrayish) -> bool {
        self.shape().iter().zip(other.shape()).all(|(a, b)| a == b)
    }
}

impl<'a, T> Arrayish for &'a T
where
    T: Arrayish,
{
    type Value = T::Value;
    fn shape(&self) -> &[usize] {
        T::shape(self)
    }
    fn data(&self) -> &[Self::Value] {
        T::data(self)
    }
}

impl<T: ArrayValue> Arrayish for Array<T> {
    type Value = T;
    fn shape(&self) -> &[usize] {
        &self.shape
    }
    fn data(&self) -> &[Self::Value] {
        &self.data
    }
}

impl<T: ArrayValue> Arrayish for (&[usize], &[T]) {
    type Value = T;
    fn shape(&self) -> &[usize] {
        self.0
    }
    fn data(&self) -> &[Self::Value] {
        self.1
    }
}

impl<T: ArrayValue> Arrayish for (&[usize], &mut [T]) {
    type Value = T;
    fn shape(&self) -> &[usize] {
        self.0
    }
    fn data(&self) -> &[Self::Value] {
        self.1
    }
}

pub trait PervasiveFn<A, B> {
    type Output;
    type Error;
    fn call(&self, a: A, b: B, env: &Uiua) -> Result<Self::Output, Self::Error>;
}

#[derive(Clone)]
pub struct InfalliblePervasiveFn<A, B, C, F>(F, PhantomData<(A, B, C)>);

impl<A, B, C, F> InfalliblePervasiveFn<A, B, C, F> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<A, B, C, F> PervasiveFn<A, B> for InfalliblePervasiveFn<A, B, C, F>
where
    F: Fn(A, B) -> C,
{
    type Output = C;
    type Error = Infallible;
    fn call(&self, a: A, b: B, _env: &Uiua) -> Result<Self::Output, Self::Error> {
        Ok((self.0)(a, b))
    }
}

#[derive(Clone)]
pub struct FalliblePerasiveFn<A, B, C, F>(F, PhantomData<(A, B, C)>);

impl<A, B, C, F> FalliblePerasiveFn<A, B, C, F> {
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<A, B, C, F> PervasiveFn<A, B> for FalliblePerasiveFn<A, B, C, F>
where
    F: Fn(A, B, &Uiua) -> UiuaResult<C>,
{
    type Output = C;
    type Error = UiuaError;
    fn call(&self, a: A, b: B, env: &Uiua) -> UiuaResult<Self::Output> {
        (self.0)(a, b, env)
    }
}

pub fn bin_pervade<A, B, C, F>(a: &Array<A>, b: &Array<B>, env: &Uiua, f: F) -> UiuaResult<Array<C>>
where
    A: ArrayValue,
    B: ArrayValue,
    C: ArrayValue,
    F: PervasiveFn<A, B, Output = C> + Clone,
    F::Error: Into<UiuaError>,
{
    let mut a = a;
    let mut b = b;
    let mut reshaped_a;
    let mut reshaped_b;
    if !a.shape_prefixes_match(&b) {
        // Fill in missing rows
        match a.row_count().cmp(&b.row_count()) {
            Ordering::Less => {
                if let Some(fill) = A::get_fill(env) {
                    let mut target_shape = a.shape().to_vec();
                    target_shape[0] = b.row_count();
                    reshaped_a = a.clone();
                    reshaped_a.fill_to_shape(&target_shape, fill);
                    a = &reshaped_a;
                }
            }
            Ordering::Greater => {
                if let Some(fill) = B::get_fill(env) {
                    let mut target_shape = b.shape().to_vec();
                    target_shape[0] = a.row_count();
                    reshaped_b = b.clone();
                    reshaped_b.fill_to_shape(&target_shape, fill);
                    b = &reshaped_b;
                }
            }
            Ordering::Equal => {}
        }
        // Fill in missing dimensions
        if !a.shape_prefixes_match(&b) {
            match a.rank().cmp(&b.rank()) {
                Ordering::Less => {
                    if let Some(fill) = A::get_fill(env) {
                        let mut target_shape = a.shape.clone();
                        target_shape.insert(0, b.row_count());
                        reshaped_a = a.clone();
                        reshaped_a.fill_to_shape(&target_shape, fill);
                        a = &reshaped_a;
                    }
                }
                Ordering::Greater => {
                    if let Some(fill) = B::get_fill(env) {
                        let mut target_shape = b.shape.clone();
                        target_shape.insert(0, a.row_count());
                        reshaped_b = b.clone();
                        reshaped_b.fill_to_shape(&target_shape, fill);
                        b = &reshaped_b;
                    }
                }
                Ordering::Equal => {
                    let target_shape = max_shape(a.shape(), b.shape());
                    if a.shape() != &*target_shape {
                        if let Some(fill) = A::get_fill(env) {
                            reshaped_a = a.clone();
                            reshaped_a.fill_to_shape(&target_shape, fill);
                            a = &reshaped_a;
                        }
                    }
                    if b.shape() != &*target_shape {
                        if let Some(fill) = B::get_fill(env) {
                            reshaped_b = b.clone();
                            reshaped_b.fill_to_shape(&target_shape, fill);
                            b = &reshaped_b;
                        }
                    }
                }
            }
            if !a.shape_prefixes_match(&b) {
                return Err(env
                    .error(format!(
                        "Shapes {} and {} do not match",
                        a.format_shape(),
                        b.format_shape()
                    ))
                    .fill());
            }
        }
    }
    let shape = Shape::from(a.shape().max(b.shape()));
    let mut data = Vec::with_capacity(a.flat_len().max(b.flat_len()));
    bin_pervade_recursive(a, b, &mut data, env, f).map_err(Into::into)?;
    Ok(Array::new(shape, data))
}

fn bin_pervade_recursive<A, B, C, F>(
    a: &A,
    b: &B,
    c: &mut Vec<C>,
    env: &Uiua,
    f: F,
) -> Result<(), F::Error>
where
    A: Arrayish,
    B: Arrayish,
    C: ArrayValue,
    F: PervasiveFn<A::Value, B::Value, Output = C> + Clone,
{
    match (a.shape(), b.shape()) {
        ([], []) => c.push(f.call(a.data()[0].clone(), b.data()[0].clone(), env)?),
        (ash, bsh) if ash == bsh => {
            for (a, b) in a.data().iter().zip(b.data()) {
                c.push(f.call(a.clone(), b.clone(), env)?);
            }
        }
        ([], bsh) => {
            for brow in b.rows() {
                bin_pervade_recursive(a, &(&bsh[1..], brow), c, env, f.clone())?;
            }
        }
        (ash, []) => {
            for arow in a.rows() {
                bin_pervade_recursive(&(&ash[1..], arow), b, c, env, f.clone())?;
            }
        }
        (ash, bsh) => {
            for (arow, brow) in a.rows().zip(b.rows()) {
                bin_pervade_recursive(&(&ash[1..], arow), &(&bsh[1..], brow), c, env, f.clone())?;
            }
        }
    }
    Ok(())
}

pub mod not {
    use super::*;
    pub fn num(a: f64) -> f64 {
        1.0 - a
    }
    pub fn byte(a: u8) -> f64 {
        num(a.into())
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot negate {a}"))
    }
}

pub mod neg {
    use super::*;
    pub fn num(a: f64) -> f64 {
        -a
    }
    pub fn byte(a: u8) -> f64 {
        -f64::from(a)
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot negate {a}"))
    }
}
pub mod abs {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.abs()
    }
    pub fn byte(a: u8) -> u8 {
        a
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot take the absolute value of {a}"))
    }
}
pub mod sign {
    use super::*;
    pub fn num(a: f64) -> f64 {
        if a.is_nan() {
            f64::NAN
        } else if a == 0.0 {
            0.0
        } else {
            a.signum()
        }
    }
    pub fn byte(a: u8) -> u8 {
        (a > 0) as u8
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the sign of {a}"))
    }
}
pub mod sqrt {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.sqrt()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).sqrt()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot take the square root of {a}"))
    }
}
pub mod sin {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.sin()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).sin()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the sine of {a}"))
    }
}
pub mod cos {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.cos()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).cos()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the cosine of {a}"))
    }
}
pub mod tan {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.tan()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).tan()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the tangent of {a}"))
    }
}
pub mod asin {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.asin()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).asin()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the arcsine of {a}"))
    }
}
pub mod acos {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.acos()
    }
    pub fn byte(a: u8) -> f64 {
        f64::from(a).acos()
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the arccosine of {a}"))
    }
}
pub mod floor {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.floor()
    }
    pub fn byte(a: u8) -> u8 {
        a
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the floor of {a}"))
    }
}
pub mod ceil {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.ceil()
    }
    pub fn byte(a: u8) -> u8 {
        a
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the ceiling of {a}"))
    }
}
pub mod round {
    use super::*;
    pub fn num(a: f64) -> f64 {
        a.round()
    }
    pub fn byte(a: u8) -> u8 {
        a
    }
    pub fn error<T: Display>(a: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the rounded value of {a}"))
    }
}

macro_rules! cmp_impl {
    ($name:ident $eq:tt $ordering:expr) => {
        pub mod $name {
            use super::*;
            pub fn always_greater<A, B>(_: A, _: B) -> u8 {
                ($ordering $eq Ordering::Less).into()
            }
            pub fn always_less<A, B>(_: A, _: B) -> u8 {
                ($ordering $eq Ordering::Greater).into()
            }
            pub fn num_num(a: f64, b: f64) -> u8 {
                (b.array_cmp(&a) $eq $ordering) as u8
            }
            pub fn byte_num(a: u8, b: f64) -> u8 {
                (b.array_cmp(&f64::from(a)) $eq $ordering) as u8
            }
            pub fn num_byte(a: f64, b: u8) -> u8 {
                (f64::from(b).array_cmp(&a) $eq $ordering) as u8
            }
            pub fn generic<T: Ord>(a: T, b: T) -> u8 {
                (b.cmp(&a) $eq $ordering).into()
            }
            pub fn error<T: Display>(a: T, b: T, _env: &Uiua) -> UiuaError {
                unreachable!("Comparisons cannot fail, failed to compare {a} and {b}")
            }
        }
    };
}

cmp_impl!(is_eq == std::cmp::Ordering::Equal);
cmp_impl!(is_ne != Ordering::Equal);
cmp_impl!(is_lt == Ordering::Less);
cmp_impl!(is_le != Ordering::Greater);
cmp_impl!(is_gt == Ordering::Greater);
cmp_impl!(is_ge != Ordering::Less);

pub mod add {

    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b + a
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(a) + f64::from(b)
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b + f64::from(a)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        a + f64::from(b)
    }
    pub fn num_char(a: f64, b: char) -> char {
        char::from_u32((b as i64 + a as i64) as u32).unwrap_or('\0')
    }
    pub fn char_num(a: char, b: f64) -> char {
        char::from_u32((b as i64 + a as i64) as u32).unwrap_or('\0')
    }
    pub fn byte_char(a: u8, b: char) -> char {
        char::from_u32((b as i64 + a as i64) as u32).unwrap_or('\0')
    }
    pub fn char_byte(a: char, b: u8) -> char {
        char::from_u32((b as i64 + a as i64) as u32).unwrap_or('\0')
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot add {a} and {b}"))
    }
}

pub mod sub {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b - a
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b) - f64::from(a)
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b - f64::from(a)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        f64::from(b) - a
    }
    pub fn num_char(a: f64, b: char) -> char {
        char::from_u32(((b as i64) - (a as i64)) as u32).unwrap_or('\0')
    }
    pub fn char_char(a: char, b: char) -> f64 {
        ((b as i64) - (a as i64)) as f64
    }
    pub fn byte_char(a: u8, b: char) -> char {
        char::from_u32(((b as i64) - (a as i64)) as u32).unwrap_or('\0')
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot subtract {a} from {b}"))
    }
}

pub mod mul {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b * a
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b) * f64::from(a)
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b * f64::from(a)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        f64::from(b) * a
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot multiply {a} and {b}"))
    }
}

pub mod div {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b / a
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b) / f64::from(a)
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b / f64::from(a)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        f64::from(b) / a
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot divide {a} by {b}"))
    }
}

pub mod modulus {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        (b % a + a) % a
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b) % f64::from(a)
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        let a = f64::from(a);
        (b % a + a) % a
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        (f64::from(b) % a + a) % a
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot take the modulus of {a} by {b}"))
    }
}

pub mod atan2 {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        a.atan2(b)
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the atan2 of {a} and {b}"))
    }
}

pub mod pow {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b.powf(a)
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b).powf(f64::from(a))
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b.powf(f64::from(a))
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        f64::from(b).powf(a)
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the power of {a} to {b}"))
    }
}

pub mod log {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        b.log(a)
    }
    pub fn byte_byte(a: u8, b: u8) -> f64 {
        f64::from(b).log(f64::from(a))
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        b.log(f64::from(a))
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        f64::from(b).log(a)
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the log base {b} of {a}"))
    }
}

pub mod max {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        a.max(b)
    }
    pub fn byte_byte(a: u8, b: u8) -> u8 {
        a.max(b)
    }
    pub fn char_char(a: char, b: char) -> char {
        a.max(b)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        num_num(a, b.into())
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        num_num(a.into(), b)
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the max of {a} and {b}"))
    }
}

pub mod min {
    use super::*;
    pub fn num_num(a: f64, b: f64) -> f64 {
        a.min(b)
    }
    pub fn byte_byte(a: u8, b: u8) -> u8 {
        a.min(b)
    }
    pub fn char_char(a: char, b: char) -> char {
        a.min(b)
    }
    pub fn num_byte(a: f64, b: u8) -> f64 {
        num_num(a, b.into())
    }
    pub fn byte_num(a: u8, b: f64) -> f64 {
        num_num(a.into(), b)
    }
    pub fn error<T: Display>(a: T, b: T, env: &Uiua) -> UiuaError {
        env.error(format!("Cannot get the min of {a} and {b}"))
    }
}

pub trait PervasiveInput: IntoIterator + Sized {
    type OwnedItem: Clone;
    fn len(&self) -> usize;
    fn only(&self) -> Self::OwnedItem;
    fn item(item: <Self as IntoIterator>::Item) -> Self::OwnedItem;
    fn as_slice(&self) -> &[Self::OwnedItem];
    fn into_only(self) -> Self::OwnedItem {
        Self::item(self.into_iter().next().unwrap())
    }
}

impl<T: Clone> PervasiveInput for Vec<T> {
    type OwnedItem = T;
    fn len(&self) -> usize {
        Vec::len(self)
    }
    fn only(&self) -> T {
        self.first().unwrap().clone()
    }
    fn item(item: <Self as IntoIterator>::Item) -> T {
        item
    }
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T: Clone> PervasiveInput for &'a [T] {
    type OwnedItem = T;
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    fn only(&self) -> T {
        self.first().unwrap().clone()
    }
    fn item(item: <Self as IntoIterator>::Item) -> T {
        item.clone()
    }
    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T: Clone> PervasiveInput for Option<T> {
    type OwnedItem = T;
    fn len(&self) -> usize {
        self.is_some() as usize
    }
    fn only(&self) -> T {
        self.as_ref().unwrap().clone()
    }
    fn item(item: <Self as IntoIterator>::Item) -> T {
        item
    }
    fn as_slice(&self) -> &[T] {
        slice::from_ref(self.as_ref().unwrap())
    }
}

pub fn bin_pervade_generic<A: PervasiveInput, B: PervasiveInput, C: Default>(
    a_shape: &[usize],
    a: A,
    b_shape: &[usize],
    b: B,
    env: &mut Uiua,
    f: impl FnMut(A::OwnedItem, B::OwnedItem, &mut Uiua) -> UiuaResult<C> + Copy,
) -> UiuaResult<(Shape, Vec<C>)> {
    let c_shape = Shape::from(cmp::max(a_shape, b_shape));
    let c_len: usize = c_shape.iter().product();
    let mut c: Vec<C> = Vec::with_capacity(c_len);
    for _ in 0..c_len {
        c.push(C::default());
    }
    bin_pervade_recursive_generic(a_shape, a, b_shape, b, &mut c, env, f)?;
    Ok((c_shape, c))
}

#[allow(unused_mut)] // for a rust-analyzer false-positive
fn bin_pervade_recursive_generic<A: PervasiveInput, B: PervasiveInput, C>(
    a_shape: &[usize],
    a: A,
    b_shape: &[usize],
    b: B,
    c: &mut [C],
    env: &mut Uiua,
    mut f: impl FnMut(A::OwnedItem, B::OwnedItem, &mut Uiua) -> UiuaResult<C> + Copy,
) -> UiuaResult {
    if a_shape == b_shape {
        for ((a, b), mut c) in a.into_iter().zip(b).zip(c) {
            *c = f(A::item(a), B::item(b), env)?;
        }
        return Ok(());
    }
    match (a_shape.is_empty(), b_shape.is_empty()) {
        (true, true) => c[0] = f(a.into_only(), b.into_only(), env)?,
        (false, true) => {
            for (a, mut c) in a.into_iter().zip(c) {
                *c = f(A::item(a), b.only(), env)?;
            }
        }
        (true, false) => {
            for (b, mut c) in b.into_iter().zip(c) {
                *c = f(a.only(), B::item(b), env)?;
            }
        }
        (false, false) => {
            let a_cells = a_shape[0];
            let b_cells = b_shape[0];
            if a_cells != b_cells {
                return Err(env.error(format!(
                    "Shapes {} and {} do not match",
                    FormatShape(a_shape),
                    FormatShape(b_shape)
                )));
            }
            let a_chunk_size = a.len() / a_cells;
            let b_chunk_size = b.len() / b_cells;
            match (a_shape.len() == 1, b_shape.len() == 1) {
                (true, true) => {
                    for ((a, b), mut c) in a.into_iter().zip(b).zip(c) {
                        *c = f(A::item(a), B::item(b), env)?;
                    }
                }
                (true, false) => {
                    for ((a, b), c) in a
                        .into_iter()
                        .zip(b.as_slice().chunks_exact(b_chunk_size))
                        .zip(c.chunks_exact_mut(b_chunk_size))
                    {
                        bin_pervade_recursive_generic(
                            &a_shape[1..],
                            Some(A::item(a)),
                            &b_shape[1..],
                            b,
                            c,
                            env,
                            f,
                        )?;
                    }
                }
                (false, true) => {
                    for ((a, b), c) in a
                        .as_slice()
                        .chunks_exact(a_chunk_size)
                        .zip(b.into_iter())
                        .zip(c.chunks_exact_mut(a_chunk_size))
                    {
                        bin_pervade_recursive_generic(
                            &a_shape[1..],
                            a,
                            &b_shape[1..],
                            Some(B::item(b)),
                            c,
                            env,
                            f,
                        )?;
                    }
                }
                (false, false) => {
                    for ((a, b), c) in a
                        .as_slice()
                        .chunks_exact(a_chunk_size)
                        .zip(b.as_slice().chunks_exact(b_chunk_size))
                        .zip(c.chunks_exact_mut(cmp::max(a_chunk_size, b_chunk_size)))
                    {
                        bin_pervade_recursive_generic(
                            &a_shape[1..],
                            a,
                            &b_shape[1..],
                            b,
                            c,
                            env,
                            f,
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}
