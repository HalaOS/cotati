use super::{Animatable, Measurement, ViewBox};
use vglang_derive::Dsl;

/// Create a new layer into which the backend render child elements.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "dsl", derive(Dsl))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    /// a number (usually an integer) that represents the width of the rendering layer.
    pub width: Animatable<Measurement>,
    /// a number (usually an integer) that represents the height of the rendering layer.
    pub height: Animatable<Measurement>,
    /// stretch to fit a particular container element.
    pub viewbox: Option<Animatable<ViewBox>>,
}

impl<W, H> From<(W, H)> for Layer
where
    Measurement: From<W> + From<H>,
{
    fn from(value: (W, H)) -> Self {
        Layer {
            width: Animatable::Constant(value.0.into()),
            height: Animatable::Constant(value.1.into()),
            viewbox: None,
        }
    }
}
