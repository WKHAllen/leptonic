#![allow(dead_code)]

use js_sys::Math;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement, InputEvent, MouseEvent};

/// Gets the value of an input element from an event.
pub fn input_event_value(event: &Event) -> String {
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a textarea element from an event.
pub fn textarea_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a content-editable element from an event.
pub fn content_editable_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlElement = event_target.dyn_into().unwrap_throw();
    target.inner_text()
}

/// Gets the value of a checkbox from a mouse click event.
pub fn checkbox_checked(e: MouseEvent) -> bool {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.checked()
}

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = Math::random().to_bits();
    let hex_value = format!("{value:x}");
    hex_value
}

/// Logs to the console.
#[allow(unused_macros)]
macro_rules! console_log {
    ( $($arg:tt)* ) => {{
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($arg)*)));
    }};
}

#[allow(unused_imports)]
pub(crate) use console_log;

/// A representation of a list of CSS classes.
pub struct Classes(Vec<String>);

impl Classes {
    /// Creates a new empty list of CSS classes.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Extends the existing list of classes with another.
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl From<&str> for Classes {
    fn from(value: &str) -> Self {
        Self(vec![value.to_owned()])
    }
}

impl From<String> for Classes {
    fn from(value: String) -> Self {
        Self(vec![value])
    }
}

impl<T> From<Vec<T>> for Classes
where
    T: Into<Classes>,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().fold(Vec::new(), |mut classes, item| {
            let item_classes: Classes = item.into();
            classes.extend(item_classes.0);
            classes
        }))
    }
}

impl<T> From<Option<T>> for Classes
where
    T: Into<Classes>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(inner) => inner.into(),
            None => Self(vec![]),
        }
    }
}

impl<F, T> From<F> for Classes
where
    F: FnOnce() -> T,
    T: Into<Classes>,
{
    fn from(value: F) -> Self {
        value().into()
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join(" "))
    }
}

macro_rules! classes {
    ( $($class:expr),* ) => {{
        #[allow(unused_mut)]
        let mut classes = $crate::util::Classes::new();
        $(
            classes.extend($crate::util::Classes::from($class));
        )*
        classes.to_string()
    }};
}

pub(crate) use classes;

/// A trait for numeric values.
pub trait Number:
    PartialEq
    + PartialOrd
    + FromStr
    + ToString
    + Default
    + Clone
    + Copy
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    const NUMBER_MIN: Self;
    const NUMBER_MAX: Self;
    const NUMBER_STEP: Self;
    const DECIMAL: bool;

    fn as_f64(self) -> f64;
}

/// Implements the `Number` trait for integer primitives.
macro_rules! impl_number_int {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = false;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

/// Implements the `Number` trait for floating point primitives.
macro_rules! impl_number_float {
    ( $($ty:ty),* ) => {
        $(
            impl Number for $ty {
                const NUMBER_MIN: Self = Self::MIN;
                const NUMBER_MAX: Self = Self::MAX;
                const NUMBER_STEP: Self = 1 as Self;
                const DECIMAL: bool = true;

                fn as_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_number_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl_number_float!(f32, f64);
