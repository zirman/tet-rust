use super::{M4, Scalar, V3};

#[derive(Clone, Debug)]
pub struct Quaternion<S: Scalar> {
    pub w: S,
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S: Scalar> Quaternion<S> {
    pub fn identity() -> Self {
        Quaternion {
            w: S::ONE,
            x: S::ZERO,
            y: S::ZERO,
            z: S::ZERO,
        }
    }
    pub fn rotation(axis: &V3<S>, angle: S) -> Self {
        Quaternion {
            w: S::cos(S::div(angle, S::TWO)),
            x: S::mul(axis.x, S::sin(S::div(angle, S::TWO))),
            y: S::mul(axis.y, S::sin(S::div(angle, S::TWO))),
            z: S::mul(axis.z, S::sin(S::div(angle, S::TWO))),
        }
    }
    pub fn to_matrix(&self) -> M4<S> {
        M4::new(
            S::sub(S::sub(S::add(S::mul(self.w, self.w), S::mul(self.x, self.x)), S::mul(self.y, self.y)), S::mul(self.z, self.z)),
            S::sub(S::mul(S::mul(S::TWO, self.x), self.y), S::mul(S::mul(S::TWO, self.w), self.z)),
            S::add(S::mul(S::mul(S::TWO, self.x), self.z), S::mul(S::mul(S::TWO, self.w), self.y)),
            S::ZERO,
            S::add(S::mul(S::mul(S::TWO, self.x), self.y), S::mul(S::mul(S::TWO, self.w), self.z)),
            S::sub(S::add(S::sub(S::mul(self.w, self.w), S::mul(self.x, self.x)), S::mul(self.y, self.y)), S::mul(self.z, self.z)),
            S::add(S::mul(S::mul(S::TWO, self.y), self.z), S::mul(S::mul(S::TWO, self.w), self.x)),
            S::ZERO,
            S::sub(S::mul(S::mul(S::TWO, self.x), self.z), S::mul(S::mul(S::TWO, self.w), self.y)),
            S::sub(S::mul(S::mul(S::TWO, self.y), self.z), S::mul(S::mul(S::TWO, self.w), self.x)),
            S::add(S::sub(S::sub(S::mul(self.w, self.w), S::mul(self.x, self.x)), S::mul(self.y, self.y)), S::mul(self.z, self.z)),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }
}
