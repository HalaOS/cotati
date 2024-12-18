use vglang_ir::IR;

use crate::generator::Generator;

/// A `Graphic` element defines a group of vector graphic instructions.
///
/// Consumes this object via [`draw`](Graphic::draw) to generate real drawing instructions.
pub trait Graphic<G>
where
    G: Generator,
{
    /// Consume self and generate real drawing instructions.
    fn draw(self, g: &mut G);
}

/// Implements [`Graphic`] for functions with signature `FnOnce(&mut G)`.
impl<G, F> Graphic<G> for F
where
    F: FnOnce(&mut G),
    G: Generator,
{
    fn draw(self, g: &mut G) {
        self(g)
    }
}

impl<G> Graphic<G> for String
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push(IR::String(self));
    }
}

impl<G> Graphic<G> for &str
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push(IR::String(self.to_owned()));
    }
}

/// Create a graphic element that create a reference instruction to animatable register.
pub fn animated<G: Generator, S: ToOwned<Owned = String>>(name: S) -> impl Graphic<G> {
    move |g: &mut G| {
        g.push(IR::Animated(name.to_owned()));
    }
}

macro_rules! tuple_drawing {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+ , G> Graphic<G> for ($header, $($tail),+)
        where
            G: Generator,
            $header: Graphic<G>,
            $($tail: Graphic<G>),+,
        {
            #[allow(non_snake_case)]
            fn draw(self, render: &mut G) {
                let ($header, $($tail),+) = self;
                $header.draw(render);
                $($tail.draw(render);)+
            }
        }

        tuple_drawing!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_drawing!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
    A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39
);

/// A scope attribute must implement this trait.
///
/// In general, a scoped instruction and a paired `pop` instruction form a scoped attribute.
pub trait Appliable {
    fn apply<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator;
}

macro_rules! tuple_appliable {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+> Appliable for ($header, $($tail),+)
        where
            $header: Appliable,
            $($tail: Appliable),+,
        {
            #[allow(non_snake_case)]
            fn apply<G,C>(self, graphic: C) -> impl Graphic<G>
            where
                C: Graphic<G>,
                G: Generator
            {
                let ($header, $($tail),+) = self;
                let graphic = $header.apply(graphic);
                $(let graphic = $tail.apply(graphic);)+

                graphic
            }
        }

        tuple_appliable!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_appliable!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
    A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39
);

/// Apply scope attributes to a `target` element.
pub fn apply<A, C, G>(attrs: A, target: C) -> impl Graphic<G>
where
    A: Appliable,
    G: Generator,
    C: Graphic<G>,
{
    attrs.apply(target)
}

/// This trait defines a graphic element that may have one/more children elements.
pub trait WithContent {
    fn with_content<G, C>(self, graphic: C) -> impl Graphic<G>
    where
        C: Graphic<G>,
        G: Generator;
}

/// apply graphic's content element.
pub fn with<P, C, G>(parent: P, content: C) -> impl Graphic<G>
where
    P: WithContent,
    G: Generator,
    C: Graphic<G>,
{
    parent.with_content(content)
}
