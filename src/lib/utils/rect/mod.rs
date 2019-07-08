use std::cmp::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect<L, B, R, T> (pub L, pub B, pub R, pub T);

impl<L, B, R, T> Rect<L, B, R, T> {
    pub fn new(left: L, bottom: B, right: R, top: T) -> Self {
        Self(left, bottom, right, top)
    }
}

impl<X, Y> Rect<X, Y, X, Y>
where X: PartialOrd, Y: PartialOrd {
    pub fn has_point(&self, x: X, y: Y) -> bool {
        let Self(left, bottom, right, top) = self;
        x >= *left && x <= *right && y >= *bottom && y <= *top
    }
}
