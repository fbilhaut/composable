//! Simple crate to allow for easy composition of "functional traits" and/or functions and/or closures.
//! 
//! Functional traits are defined by the [`Composable`] trait and its `apply` method.
//! 
//! Two or more composables can be composed through different means:
//! * the [`Composable::compose`] method,
//! * the [`compose`] function,
//! * the [`composed!`] macro.
//! 
//! Basic example:
//! 
//! ```
//! use composable::*;
//! 
//! struct AddTo { addend: usize }
//! 
//! impl Composable<usize, usize> for AddTo {
//!     fn apply(&self, input: usize) -> Result<usize> { Ok(input + self.addend) }
//! }
//! 
//! fn demo() -> Result<()> {
//!     let increment = AddTo { addend: 1 };
//!     let square = |x: usize| Ok(x * x);
//!     let composition = compose(increment, square);
//!     let result = composition.apply(2)?;
//!     assert_eq!(result, 9);
//!     Ok(())
//! }
//! ```
//! 
//! It is also possible to combine a composable of type `X->(Y, V1)` with another 
//! composable of type `Y->(Z, V2)` into a composable of type `X->(Z, (V1, V2))`.
//! This operation is provided by [`compose_t()`] and [`composed_t!`].
//! 
//! For example:
//! 
//! ```
//! use composable::*;
//! 
//! struct AddToMsg { addend: usize }
//! 
//! impl Composable<usize, (usize, String)> for AddToMsg {
//!     fn apply(&self, input: usize) -> Result<(usize, String)> { 
//!         Ok((input + self.addend, "hello".to_string())) 
//!     }
//! }
//! 
//! fn demo_t() -> Result<()> {
//!     let increment = AddToMsg { addend: 1 };
//!     let square = |x: usize| Ok(((x * x), "world".to_string()));
//!     let composition = compose_t(increment, square);
//!     let (result, (msg1, msg2)) = composition.apply(2)?;
//!     assert_eq!(result, 9);
//!     assert_eq!(msg1, "hello");
//!     assert_eq!(msg2, "world");
//!     Ok(())
//! }
//! ```
//! 
//! The symmetric operation is provided by [`compose_rt()`] and the [`composed_rt!`] macro.

use std::{fmt::Display, marker::PhantomData};

#[cfg(test)] mod tests;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


/// Defines a "functional" (composable) trait
pub trait Composable<I, O> {
    fn apply(&self, input: I) -> Result<O>;

    fn compose<T, P>(self, other: T) -> impl Composable<I, P> where 
        Self: Sized,
        T: Composable<O, P>,
    {
        compose(self, other)
    }
}

/// Makes any `Fn<I, Result<O>>` composable
impl<F, I, O> Composable<I, O> for F where 
    F: Fn(I) -> Result<O> 
{
    fn apply(&self, arg: I) -> Result<O> {
        self(arg)
    }
}

/// Composition
pub fn compose<T1, T2, I, O, X>(t1: T1, t2: T2) -> impl Composable<I, O> where 
    T1: Composable<I, X>,
    T2: Composable<X, O>,
{
    Composed::new(t1, t2)
}


/// Support for composition 
struct Composed<T1, T2, X> {
    t1: T1,
    t2: T2,
    _marker: PhantomData<X>,
}

impl<T1, T2, X> Composed<T1, T2, X> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2, _marker: PhantomData }
    }
}

impl<T1, T2, I, O, X> Composable<I, O> for Composed<T1, T2, X> where 
    T1: Composable<I, X>,
    T2: Composable<X, O>,
{
    fn apply(&self, input: I) -> Result<O> {
        self.t2.apply(self.t1.apply(input)?)
    }
}

/// Special composition that combines tuple output values
/// 
/// Example: 
/// * Composable: `X->(Y, V1)`
/// * And composable: `Y->(Z, V2)`
/// * Are composed into: `X->(Z, (V1, V2))`
pub fn compose_t<T1, T2, I, O, V1, V2, X>(t1: T1, t2: T2) -> impl Composable<I, (O, (V1, V2))> where
    T1: Composable<I, (X, V1)>,
    T2: Composable<X, (O, V2)>,
{
    ComposedTuples::new(t1, t2)
}

struct ComposedTuples<T1, T2, X> {
    t1: T1,
    t2: T2,
    _marker: PhantomData<X>,
}

impl<T1, T2, X> ComposedTuples<T1, T2, X> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2, _marker: PhantomData }
    }
}

impl<T1, T2, I, O, V1, V2, X> Composable<I, (O, (V1, V2))> for ComposedTuples<T1, T2, X> where 
    T1: Composable<I, (X, V1)>,
    T2: Composable<X, (O, V2)>,
{
    fn apply(&self, input: I) -> Result<(O, (V1, V2))> {
        let (x, v1) = self.t1.apply(input)?;
        let (o, v2) = self.t2.apply(x)?;
        Ok((o, (v1, v2)))
    }
}


/// Special composition that combines tuple input values
/// 
/// Example: 
/// * Composable: `(X, V1)->Y`
/// * And composable: `(Y, V2)->Z`
/// * Are composed into: `(X, (V2, V1))->Z`
pub fn compose_rt<T1, T2, I, O, V1, V2, X>(t1: T1, t2: T2) -> impl Composable<(I, (V2, V1)), O> where
    T1: Composable<(I, V1), X>,
    T2: Composable<(X, V2), O>,
{
    ComposedTuplesR::new(t1, t2)
}

struct ComposedTuplesR<T1, T2, X> {
    t1: T1,
    t2: T2,
    _marker: PhantomData<X>,
}

impl<T1, T2, X> ComposedTuplesR<T1, T2, X> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2, _marker: PhantomData }
    }
}

impl<T1, T2, I, O, V1, V2, X> Composable<(I, (V2, V1)), O> for ComposedTuplesR<T1, T2, X> where 
    T1: Composable<(I, V1), X>,
    T2: Composable<(X, V2), O>,
{
    fn apply(&self, input: (I, (V2, V1))) -> Result<O> {
        let (i, (v2, v1)) = input;
        let x = self.t1.apply((i, v1))?;
        let o = self.t2.apply((x, v2))?;
        Ok(o)
    }
}



/// Utility `Composable` that prints the passed value and returns it untouched
pub struct Print {
    prefix: Option<String>,
    suffix: Option<String>,
}

impl Print {
    pub fn new<S: Into<String>>(prefix: Option<S>, suffix: Option<S>) -> Self {
        Self { 
            prefix: prefix.map(|x| x.into()),
            suffix: suffix.map(|x| x.into()),
        }
    }
}

impl Default for Print {
    fn default() -> Self {
        Self { prefix: None, suffix: None }
    }
}

impl<T: Display> Composable<T, T> for Print {
    fn apply(&self, input: T) -> Result<T> {
        if let Some(prefix) = &self.prefix { print!("{prefix}") }
        print!("{}", &input);
        if let Some(suffix) = &self.suffix { print!("{suffix}") }
        Ok(input)
    }
}


/// Macro-rules for easy composition (owned)
#[macro_export]
macro_rules! composed {
    ($e:expr) => {{
        $e
    }};

    ($e:expr, $($es:expr),+ ) => {{
        let t1 = $crate::composed! { $e };
        let t2 = $crate::composed! { $($es),+ };
        $crate::compose(t1, t2)
    }};
}


/// Macro-rules for easy composition (with output tuples)
#[macro_export]
macro_rules! composed_t {
    ($e:expr) => {{
        $e
    }};

    ($e:expr, $($es:expr),+ ) => {{
        let t1 = $crate::composed_t! { $e };
        let t2 = $crate::composed_t! { $($es),+ };
        $crate::compose_t(t1, t2)
    }};
}




/// Macro-rules for easy composition (with input tuples)
#[macro_export]
macro_rules! composed_rt {
    ($e:expr) => {{
        $e
    }};

    ($e:expr, $($es:expr),+ ) => {{
        let t1 = $crate::composed_rt! { $e };
        let t2 = $crate::composed_rt! { $($es),+ };
        $crate::compose_rt(t1, t2)
    }};
}

