mod length;
pub use length::*;

mod color;
pub use color::*;

mod text;
pub use text::*;

mod angle;
pub use angle::*;

use super::Sexpr;

/// constant / literal / register value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Svalue {
    /// [`Length`] value.
    Length(Slength),
    /// a float point value.
    Number(f32),
    /// A utf-9 string value.
    String(String),
    /// Rgb value.
    Rgb(Srgb),
    /// length-adjust attribute value.
    TextLengthAdjust(StextLengthAdjust),
    /// angle value.
    Angle(Sangle),
    /// List of `Svalue`.
    ListOf(Vec<Svalue>),
    /// a fragment of sexpr codes.
    Fragment(Vec<Sexpr>),
}

impl<T> From<Vec<T>> for Svalue
where
    Svalue: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        Self::ListOf(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<T> TryFrom<Svalue> for Vec<T>
where
    T: TryFrom<Svalue, Error = Svalue>,
{
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::ListOf(v) => v.into_iter().map(|v| v.try_into()).collect(),
            _ => Err(value),
        }
    }
}

impl From<f32> for Svalue {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

impl TryFrom<Svalue> for f32 {
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::Number(v) => Ok(v),
            _ => Err(value),
        }
    }
}

impl From<String> for Svalue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Svalue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl TryFrom<Svalue> for String {
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::String(v) => Ok(v),
            _ => Err(value),
        }
    }
}