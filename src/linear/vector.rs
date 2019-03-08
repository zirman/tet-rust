use super::Scalar;

#[derive(Copy, Clone, Debug)]
pub struct V2<S: Scalar> {
    pub x: S,
    pub y: S,
}

impl<S: Scalar> V2<S> {
    pub fn new(x: S, y: S) -> Self {
        Self { x: x, y: y }
    }
    pub fn dot(lhs: &Self, rhs: &Self) -> S {
        S::add(S::mul(lhs.x, rhs.x), S::mul(lhs.y, rhs.y))
    }
    pub fn neg(v: &Self) -> Self {
        Self::new(S::neg(v.x), S::neg(v.y))
    }
    pub fn add(lhs: &Self, rhs: &Self) -> Self {
        Self::new(S::add(lhs.x, rhs.x), S::add(lhs.y, rhs.y))
    }
    pub fn sub(lhs: &Self, rhs: &Self) -> Self {
        Self::new(S::sub(lhs.x, rhs.x), S::sub(lhs.y, rhs.y))
    }
    pub fn mul(v: &Self, s: S) -> Self {
        Self::new(S::mul(v.x, s), S::mul(v.y, s))
    }
    pub fn sqr_len(v: &Self) -> S {
        Self::dot(v, v)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct V3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S: Scalar> V3<S> {
    pub fn new(x: S, y: S, z: S) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn dot(lhs: &Self, rhs: &Self) -> S {
        S::add(S::add(S::mul(lhs.x, rhs.x), S::mul(lhs.y, rhs.y)), S::mul(lhs.z, rhs.z))
    }
    pub fn neg(v: &Self) -> Self {
        Self::new(S::neg(v.x), S::neg(v.y), S::neg(v.z))
    }
    pub fn add(lhs: &Self, rhs: &Self) -> Self {
        Self::new(S::add(lhs.x, rhs.x), S::add(lhs.y, rhs.y), S::add(lhs.z, rhs.z))
    }
    pub fn sub(lhs: &Self, rhs: &Self) -> Self {
        Self::new(S::sub(lhs.x, rhs.x), S::sub(lhs.y, rhs.y), S::sub(lhs.z, rhs.z))
    }
    pub fn mul(v: &Self, s: S) -> Self {
        Self::new(S::mul(v.x, s), S::mul(v.y, s), S::mul(v.z, s))
    }
    pub fn sqr_len(v: &Self) -> S {
        Self::dot(v, v)
    }
    pub fn cros(lhs: &Self, rhs: &Self) -> Self {
        Self::new(
            S::sub(S::mul(lhs.y, rhs.z), S::mul(rhs.y, lhs.z)),
            S::sub(S::mul(lhs.z, rhs.x), S::mul(rhs.z, lhs.x)),
            S::sub(S::mul(lhs.x, rhs.y), S::mul(rhs.x, lhs.y)),
        )
    }
    pub fn to_v4(v: &Self) -> V4<S> {
        V4::new(v.x, v.y, v.z, S::ONE)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct V4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<S: Scalar> V4<S> {
    pub fn new(x: S, y: S, z: S, w: S) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn dot(lhs: &Self, rhs: &Self) -> S {
        S::add(S::add(S::add(S::mul(lhs.x, rhs.x), S::mul(lhs.y, rhs.y)), S::mul(lhs.z, rhs.z)), S::mul(lhs.w, rhs.w))
    }
    pub fn neg(v: &Self) -> Self {
        Self::new(S::neg(v.x), S::neg(v.y), S::neg(v.z), S::neg(v.w))
    }
    pub fn add(lhs: &Self, rhs: &Self) -> Self {
        Self::new(
            S::add(lhs.x, rhs.x),
            S::add(lhs.y, rhs.y),
            S::add(lhs.z, rhs.z),
            S::add(lhs.w, rhs.w),
        )
    }
    pub fn sub(lhs: &Self, rhs: &Self) -> Self {
        Self::new(
            S::sub(lhs.x, rhs.x),
            S::sub(lhs.y, rhs.y),
            S::sub(lhs.z, rhs.z),
            S::sub(lhs.w, rhs.w),
        )
    }
    pub fn mul(v: &Self, s: S) -> Self {
        Self::new(S::mul(v.x, s), S::mul(v.y, s), S::mul(v.z, s), S::mul(v.w, s))
    }
    pub fn sqr_len(v: &Self) -> S {
        Self::dot(v, v)
    }
    pub fn to_v2(v: &Self) -> V2<S> {
        V2::new(S::div(v.x, v.w), S::div(v.y, v.w))
    }
    pub fn norm(v: &Self) -> Self {
        Self::new(S::div(v.x, v.w), S::div(v.y, v.w), S::div(v.z, v.w), S::ONE)
    }
}
