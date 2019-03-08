use super::{Scalar, V4};

#[derive(Debug)]
pub struct M4<S: Scalar> {
    pub r1: V4<S>,
    pub r2: V4<S>,
    pub r3: V4<S>,
    pub r4: V4<S>,
}

impl<S: Scalar> M4<S> {
    pub fn new(
        m11: S,
        m12: S,
        m13: S,
        m14: S,
        m21: S,
        m22: S,
        m23: S,
        m24: S,
        m31: S,
        m32: S,
        m33: S,
        m34: S,
        m41: S,
        m42: S,
        m43: S,
        m44: S,
    ) -> M4<S> {
        M4 {
            r1: V4::new(m11, m12, m13, m14),
            r2: V4::new(m21, m22, m23, m24),
            r3: V4::new(m31, m32, m33, m34),
            r4: V4::new(m41, m42, m43, m44),
        }
    }
    pub fn c1(&self) -> V4<S> {
        V4::new(self.r1.x, self.r2.x, self.r3.x, self.r4.x)
    }
    pub fn c2(&self) -> V4<S> {
        V4::new(self.r1.y, self.r2.y, self.r3.y, self.r4.y)
    }
    pub fn c3(&self) -> V4<S> {
        V4::new(self.r1.z, self.r2.z, self.r3.z, self.r4.z)
    }
    pub fn c4(&self) -> V4<S> {
        V4::new(self.r1.w, self.r2.w, self.r3.w, self.r4.w)
    }
    pub fn transpose(m: &Self) -> Self {
        Self::new(
            m.r1.x, m.r2.x, m.r3.x, m.r4.x, m.r1.y, m.r2.y, m.r3.y, m.r4.y,
            m.r1.z, m.r2.z, m.r3.z, m.r4.z, m.r1.w, m.r2.w, m.r3.w, m.r4.w,
        )
    }
    pub fn mul(m: &Self, v: V4<S>) -> V4<S> {
        V4::new(
            V4::dot(&m.r1, &v),
            V4::dot(&m.r2, &v),
            V4::dot(&m.r3, &v),
            V4::dot(&m.r4, &v),
        )
    }
    pub fn mult(lhs: Self, rhs: Self) -> Self {
        let c1 = rhs.c1();
        let c2 = rhs.c2();
        let c3 = rhs.c3();
        let c4 = rhs.c4();

        Self::new(
            V4::dot(&lhs.r1, &c1),
            V4::dot(&lhs.r1, &c2),
            V4::dot(&lhs.r1, &c3),
            V4::dot(&lhs.r1, &c4),
            V4::dot(&lhs.r2, &c1),
            V4::dot(&lhs.r2, &c2),
            V4::dot(&lhs.r2, &c3),
            V4::dot(&lhs.r2, &c4),
            V4::dot(&lhs.r3, &c1),
            V4::dot(&lhs.r3, &c2),
            V4::dot(&lhs.r3, &c3),
            V4::dot(&lhs.r3, &c4),
            V4::dot(&lhs.r4, &c1),
            V4::dot(&lhs.r4, &c2),
            V4::dot(&lhs.r4, &c3),
            V4::dot(&lhs.r4, &c4),
        )
    }

    pub fn identity() -> Self {
        Self::new(
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn orthographic_proj(l: S, r: S, b: S, t: S, n: S, f: S) -> Self {
        Self::new(
            S::div(S::TWO, S::sub(r, l)),
            S::ZERO,
            S::ZERO,
            S::div(S::add(r, l), S::sub(r, l)),
            S::ZERO,
            S::div(S::TWO, S::sub(t, b)),
            S::ZERO,
            S::div(S::add(t, b), S::sub(t, b)),
            S::ZERO,
            S::ZERO,
            S::neg(S::div(S::TWO, S::sub(f, n))),
            S::div(S::add(f, n), S::sub(f, n)),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn orthographic_proj_inv(l: S, r: S, b: S, t: S, n: S, f: S) -> Self {
        Self::new(
            S::div(S::TWO, S::sub(r, l)),
            S::ZERO,
            S::ZERO,
            S::div(S::add(r, l), S::sub(r, l)),
            S::ZERO,
            S::div(S::TWO, S::sub(t, b)),
            S::ZERO,
            S::div(S::add(t, b), S::sub(t, b)),
            S::ZERO,
            S::ZERO,
            S::neg(S::div(S::TWO, S::sub(f, n))),
            S::div(S::add(f, n), S::sub(f, n)),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn perspective_proj(l: S, r: S, b: S, t: S, n: S, f: S) -> Self {
        Self::new(
            S::div(S::mul(S::TWO, n), S::sub(r, l)),
            S::ZERO,
            S::div(S::add(r, l), S::sub(r, l)),
            S::ZERO,
            S::ZERO,
            S::div(S::mul(S::TWO, n), S::sub(t, b)),
            S::div(S::add(t, b), S::sub(t, b)),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::neg(S::div(S::add(f, n), S::sub(f, n))),
            S::neg(S::div(S::mul(S::mul(S::TWO, f), n), S::sub(f, n))),
            S::ZERO,
            S::ZERO,
            S::neg(S::ONE),
            S::ZERO,
        )
    }

    pub fn perspective_proj_inv(l: S, r: S, b: S, t: S, n: S, f: S) -> Self {
        Self::new(
            S::div(S::sub(r, l), S::mul(S::TWO, n)),
            S::ZERO,
            S::ZERO,
            S::div(S::add(r, l), S::mul(S::TWO, n)),
            S::ZERO,
            S::div(S::sub(t, b), S::mul(S::TWO, n)),
            S::ZERO,
            S::div(S::add(t, b), S::mul(S::TWO, n)),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::neg(S::ONE),
            S::ZERO,
            S::ZERO,
            S::neg(S::div(S::sub(f, n), S::mul(S::mul(S::TWO, f), n))),
            S::div(S::add(f, n), S::mul(S::mul(S::TWO, f), n)),
        )
    }

    pub fn translate(x: S, y: S, z: S) -> Self {
        Self::new(
            S::ONE,
            S::ZERO,
            S::ZERO,
            x,
            S::ZERO,
            S::ONE,
            S::ZERO,
            y,
            S::ZERO,
            S::ZERO,
            S::ONE,
            z,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn translate_inv(x: S, y: S, z: S) -> Self {
        Self::new(
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::neg(x),
            S::ZERO,
            S::ONE,
            S::ZERO,
            S::neg(y),
            S::ZERO,
            S::ZERO,
            S::ONE,
            S::neg(z),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn scale(x: S, y: S, z: S) -> Self {
        Self::new(
            x,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            y,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            z,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn scale_inv(x: S, y: S, z: S) -> Self {
        Self::new(
            S::div(S::ONE, x),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::div(S::ONE, y),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::div(S::ONE, z),
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
        )
    }

    pub fn viewport(width: S, height: S) -> Self {
        let width2 = S::div(width, S::TWO);
        let height2 = S::div(height, S::TWO);
        Self::mult(
            Self::translate(width2, height2, S::ZERO),
            Self::mult(
                Self::scale(width2, height2, S::ONE),
                Self::scale(S::ONE, S::neg(S::ONE), S::ONE)
            ),
        )
    }

    pub fn perspective(fov: S, aspect_ratio: S, n: S, f: S) -> Self {
        // S::mul(S::tan(S::rads(S::div(fov, S::TWO))), n);
        let scale = S::mul(S::tan(S::rads(S::div(fov, S::TWO))), n);
        let r = S::mul(aspect_ratio, scale);
        let l = S::neg(r);
        let t = scale;
        let b = S::neg(t);
        Self::perspective_proj(l, r, b, t, n, f)
    }
}
