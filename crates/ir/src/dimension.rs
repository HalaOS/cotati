use std::{f32::consts::PI, fmt::Display};

use crate::{tuple_map_collect, MapCollect};

use super::{Animatable, FrameVariable};

/// The unit identifier.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Unit {
    /// The 'font-size' of the relevant font
    Em,
    /// The 'ex' unit is defined by the font's 'x-height'.
    Ex,
    /// pixels, relative to the viewing device
    Px,
    /// 1 inch is equal to 2.54 centimeters.
    In,
    /// centimeters
    Cm,
    /// millimeters
    Mm,
    /// the points used by CSS2 are equal to 1/72th of an inch.
    Pt,
    /// 1 pica is equal to 12 points.
    Pc,
    /// The meaning of a percentage measurement value depends on the attribute for which the percentage measurement value has been specified.
    Percentages,
}

impl AsRef<str> for Unit {
    fn as_ref(&self) -> &str {
        match self {
            Unit::Em => "em",
            Unit::Ex => "ex",
            Unit::Px => "px",
            Unit::In => "in",
            Unit::Cm => "cm",
            Unit::Mm => "mm",
            Unit::Pt => "pt",
            Unit::Pc => "pc",
            Unit::Percentages => "%",
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// measurement, given as a number along with a unit.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Measurement(pub f32, pub Option<Unit>);

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unit) = self.1 {
            write!(f, "{}{}", self.0, unit)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

/// Measurement can be used as context variant type.
impl FrameVariable for Measurement {}

impl Default for Measurement {
    fn default() -> Self {
        Self(0f32, None)
    }
}

impl From<f32> for Measurement {
    fn from(value: f32) -> Self {
        Self(value, None)
    }
}

impl From<i32> for Measurement {
    fn from(value: i32) -> Self {
        Self(value as f32, None)
    }
}

impl Measurement {
    /// Create a measurement with `em` unit identifier.
    pub fn em(value: f32) -> Self {
        Self(value, Some(Unit::Em))
    }

    /// Create a measurement with `em` unit identifier.
    pub fn ex(value: f32) -> Self {
        Self(value, Some(Unit::Ex))
    }

    /// Create a measurement with `px` unit identifier.
    pub fn px(value: f32) -> Self {
        Self(value, Some(Unit::Px))
    }

    /// Create a measurement with `inch` unit identifier.
    pub fn inch(value: f32) -> Self {
        Self(value, Some(Unit::In))
    }
    /// Create a measurement with `cm` unit identifier.
    pub fn cm(value: f32) -> Self {
        Self(value, Some(Unit::Cm))
    }
    /// Create a measurement with `mm` unit identifier.
    pub fn mm(value: f32) -> Self {
        Self(value, Some(Unit::Mm))
    }
    /// Create a measurement with `pt` unit identifier.
    pub fn pt(value: f32) -> Self {
        Self(value, Some(Unit::Pt))
    }
    /// Create a measurement with `pc` unit identifier.
    pub fn pc(value: f32) -> Self {
        Self(value, Some(Unit::Pc))
    }

    /// Create a measurement with `px` unit identifier.
    pub fn percentage(value: f32) -> Self {
        Self(value.into(), Some(Unit::Percentages))
    }
}

/// see [`svg`] document for more information.
///
/// [`svg`]: https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet,
    Slice,
}

impl Display for MeetOrSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeetOrSlice::Meet => write!(f, "meet"),
            MeetOrSlice::Slice => write!(f, "slice"),
        }
    }
}

/// In some cases, typically when using the ‘viewBox’ attribute, i
/// t is desirable that the graphics stretch to fit non-uniformly
/// to take up the entire viewport. In other cases, it is desirable
/// that uniform scaling be used for the purposes of preserving
/// the aspect ratio of the graphics.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    /// Force uniform scaling
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMinYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMidYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMaxYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMinYMid(MeetOrSlice),
    /// Force uniform scaling(the default).
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMidYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMaxYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMinYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMidYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMaxYMax(MeetOrSlice),
}

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self::xMidYMid(MeetOrSlice::Meet)
    }
}

/// PreserveAspectRatio can be used as context variant type.
impl FrameVariable for PreserveAspectRatio {}

/// Angles are specified in one of two ways depending upon
/// whether they are used in CSS property syntax or SVG
/// presentation attribute syntax:
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Angle {
    deg(f32),
    grad(f32),
    rad(f32),
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::deg(v) => write!(f, "{}deg", v),
            Angle::grad(v) => write!(f, "{}grad", v),
            Angle::rad(v) => write!(f, "{}rad", v),
        }
    }
}

impl Angle {
    pub fn as_deg(&self) -> f32 {
        match self {
            Angle::deg(v) => *v,
            Angle::grad(v) => *v * 360.0 / 400.0,
            Angle::rad(v) => *v * 180.0 / PI,
        }
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Self::deg(value)
    }
}

impl From<i32> for Angle {
    fn from(value: i32) -> Self {
        Self::deg(value as f32)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::deg(0.0)
    }
}

/// Angle can be used as context variant type.
impl FrameVariable for Angle {}

impl Angle {
    /// Create instance of `angle=0.0deg`.
    pub fn zero() -> Self {
        Self::deg(0.0)
    }

    /// Create instance of `angle=2πdeg`.
    pub fn two_pi() -> Self {
        Self::deg(360.0)
    }

    /// Create instance of `angle=πdeg`.
    pub fn pi() -> Self {
        Self::deg(180.0)
    }
}

/// A 2d coordinate point.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: Measurement,
    pub y: Measurement,
}

/// Point can be used as context variant type.
impl FrameVariable for Point {}

/// Create a point from (f32,f32) with default unit `px`.
impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl Point {
    /// Create a point with `em` unit identifier.
    pub fn em(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::em(x),
            y: Measurement::em(y),
        }
    }

    /// Create a point with `em` unit identifier.
    pub fn ex(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::ex(x),
            y: Measurement::ex(y),
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn px(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::px(x),
            y: Measurement::px(y),
        }
    }

    /// Create a point with `inch` unit identifier.
    pub fn inch(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::inch(x),
            y: Measurement::inch(y),
        }
    }
    /// Create a point with `cm` unit identifier.
    pub fn cm(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::cm(x),
            y: Measurement::cm(y),
        }
    }
    /// Create a point with `mm` unit identifier.
    pub fn mm(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::mm(x),
            y: Measurement::mm(y),
        }
    }
    /// Create a point with `pt` unit identifier.
    pub fn pt(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::pt(x),
            y: Measurement::pt(y),
        }
    }
    /// Create a point with `pc` unit identifier.
    pub fn pc(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::pc(x),
            y: Measurement::pc(y),
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn percentage(x: f32, y: f32) -> Self {
        Self {
            x: Measurement::percentage(x),
            y: Measurement::percentage(y),
        }
    }
}

/// It is often desirable to specify that a given set of graphics stretch to fit a particular container element.
/// The ‘viewBox’ attribute provides this capability.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "dsl", derive(vglang_derive::Dsl))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBox {
    /// ViewBox left-top x coordinate,
    pub minx: Animatable<Measurement>,
    /// ViewBox left-top y coordinate,
    pub miny: Animatable<Measurement>,
    /// ViewBox width dimension.
    pub width: Animatable<Measurement>,
    /// ViewBox height dimension.
    pub height: Animatable<Measurement>,
    /// clip preserve aspect ratio.
    pub aspect: Option<Animatable<PreserveAspectRatio>>,
}

impl FrameVariable for ViewBox {}

impl<X, Y, W, H> From<(X, Y, W, H)> for ViewBox
where
    Measurement: From<X> + From<Y> + From<W> + From<H>,
{
    fn from(value: (X, Y, W, H)) -> Self {
        Self {
            minx: Animatable::Constant(value.0.into()),
            miny: Animatable::Constant(value.1.into()),
            width: Animatable::Constant(value.2.into()),
            height: Animatable::Constant(value.3.into()),
            aspect: None,
        }
    }
}

impl<X, Y, W, H, P> From<(X, Y, W, H, P)> for ViewBox
where
    Measurement: From<X> + From<Y> + From<W> + From<H>,
    PreserveAspectRatio: From<P>,
{
    fn from(value: (X, Y, W, H, P)) -> Self {
        Self {
            minx: Animatable::Constant(value.0.into()),
            miny: Animatable::Constant(value.1.into()),
            width: Animatable::Constant(value.2.into()),
            height: Animatable::Constant(value.3.into()),
            aspect: Some(Animatable::Constant(value.4.into())),
        }
    }
}

#[cfg(feature = "dsl")]
mod dsl {
    use super::*;

    impl MapCollect<Measurement> for Vec<f32> {
        fn map_collect(self) -> Vec<Measurement> {
            self.into_iter().map(|v| v.into()).collect()
        }
    }

    impl MapCollect<Measurement> for Vec<i32> {
        fn map_collect(self) -> Vec<Measurement> {
            self.into_iter().map(|v| v.into()).collect()
        }
    }

    impl MapCollect<Angle> for Vec<f32> {
        fn map_collect(self) -> Vec<Angle> {
            self.into_iter().map(|v| Angle::deg(v)).collect()
        }
    }

    impl MapCollect<Angle> for Vec<i32> {
        fn map_collect(self) -> Vec<Angle> {
            self.into_iter().map(|v| Angle::deg(v as f32)).collect()
        }
    }

    tuple_map_collect!(
        Measurement,
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        A17,
        A18,
        A19,
        A20
    );

    tuple_map_collect!(
        Angle, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18,
        A19, A20
    );
}
