use super::V4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct M4 {
    arr: [f32; 16],
}

impl M4 {
    pub fn new(
        m11: f32,
        m12: f32,
        m13: f32,
        m14: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m24: f32,
        m31: f32,
        m32: f32,
        m33: f32,
        m34: f32,
        m41: f32,
        m42: f32,
        m43: f32,
        m44: f32,
    ) -> Self {
        M4 {
            arr: [
                m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44,
            ],
        }
    }
    pub fn array(&self) -> &[f32; 16] {
        &self.arr
    }
    pub fn r1(&self) -> V4 {
        V4::new(self.arr[0], self.arr[1], self.arr[2], self.arr[3])
    }
    pub fn r2(&self) -> V4 {
        V4::new(self.arr[4], self.arr[5], self.arr[6], self.arr[7])
    }
    pub fn r3(&self) -> V4 {
        V4::new(self.arr[8], self.arr[9], self.arr[10], self.arr[11])
    }
    pub fn r4(&self) -> V4 {
        V4::new(self.arr[12], self.arr[13], self.arr[14], self.arr[15])
    }
    pub fn c1(&self) -> V4 {
        V4::new(self.arr[0], self.arr[4], self.arr[8], self.arr[12])
    }
    pub fn c2(&self) -> V4 {
        V4::new(self.arr[1], self.arr[5], self.arr[9], self.arr[13])
    }
    pub fn c3(&self) -> V4 {
        V4::new(self.arr[2], self.arr[6], self.arr[10], self.arr[14])
    }
    pub fn c4(&self) -> V4 {
        V4::new(self.arr[3], self.arr[7], self.arr[11], self.arr[15])
    }
    pub fn transpose(&self) -> Self {
        M4 {
            arr: [
                self.arr[0],
                self.arr[4],
                self.arr[8],
                self.arr[12],
                self.arr[1],
                self.arr[5],
                self.arr[9],
                self.arr[13],
                self.arr[2],
                self.arr[6],
                self.arr[10],
                self.arr[14],
                self.arr[3],
                self.arr[7],
                self.arr[11],
                self.arr[15],
            ],
        }
    }
    pub fn mul(&self, v: &V4) -> V4 {
        V4::new(
            self.r1().dot(&v),
            self.r2().dot(&v),
            self.r3().dot(&v),
            self.r4().dot(&v),
        )
    }
    pub fn mult(&self, rhs: &Self) -> Self {
        let r1 = self.r1();
        let r2 = self.r2();
        let r3 = self.r3();
        let r4 = self.r4();
        let c1 = rhs.c1();
        let c2 = rhs.c2();
        let c3 = rhs.c3();
        let c4 = rhs.c4();

        M4::new(
            r1.dot(&c1),
            r1.dot(&c2),
            r1.dot(&c3),
            r1.dot(&c4),
            r2.dot(&c1),
            r2.dot(&c2),
            r2.dot(&c3),
            r2.dot(&c4),
            r3.dot(&c1),
            r3.dot(&c2),
            r3.dot(&c3),
            r3.dot(&c4),
            r4.dot(&c1),
            r4.dot(&c2),
            r4.dot(&c3),
            r4.dot(&c4),
        )
    }

    pub fn identity() -> Self {
        M4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn orthographic_proj_2d(l: f32, r: f32, b: f32, t: f32) -> Self {
        M4::new(
            2.0 / (r - l),
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / (t - b),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            -(r + l) / (r - l),
            -(t + b) / (t - b),
            0.0,
            1.0,
        )
    }

    pub fn orthographic_proj_inv(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        M4::new(
            2.0 / (r - l),
            0.0,
            0.0,
            (r + l) / (r - l),
            0.0,
            2.0 / (t - b),
            0.0,
            (t + b) / (t - b),
            0.0,
            0.0,
            -2.0 / (f - n),
            (f + n) / (f - n),
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn perspective_proj(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        M4::new(
            (2.0 * n) / (r - l),
            0.0,
            (r + l) / (r - l),
            0.0,
            0.0,
            (2.0 * n) / (t - b),
            (t + b) / (t - b),
            0.0,
            0.0,
            0.0,
            -(f + n) / (f - n),
            (-2.0 * f * n) / (f - n),
            0.0,
            0.0,
            -1.0,
            0.0,
        )
    }

    pub fn perspective_proj_inv(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        M4::new(
            (r - l) / (2.0 * n),
            0.0,
            0.0,
            (r + l) / (2.0 * n),
            0.0,
            (t - b) / (2.0 * n),
            0.0,
            (t + b) / (2.0 * n),
            0.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            0.0,
            -(f - n) / (2.0 * f * n),
            (f + n) / (2.0 * f * n),
        )
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        M4::new(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translate_inv(x: f32, y: f32, z: f32) -> Self {
        M4::new(
            1.0, 0.0, 0.0, -x, 0.0, 1.0, 0.0, -y, 0.0, 0.0, 1.0, -z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        M4::new(
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scale_inv(x: f32, y: f32, z: f32) -> Self {
        M4::new(
            1.0 / x,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    // fn viewport(width: f32, height: f32) -> Self {
    //     let width2 = width * 0.5;
    //     let height2 = height * 0.5;
    //     Self::mult(
    //         Self::translate(width2, height2, 0.0),
    //         Self::mult(
    //             Self::scale(width2, height2, 1.0),
    //             Self::scale(1.0, -1.0, 1.0)
    //         ),
    //     )
    // }

    pub fn perspective(fov: f32, aspect_ratio: f32, n: f32, f: f32) -> Self {
        // S::mul(S::tan(S::rads(S::div(fov, 2.0))), n);
        let scale = (fov / 2.0).to_radians().tan() * n;
        let r = aspect_ratio * scale;
        let l = -r;
        let t = scale;
        let b = -t;
        Self::perspective_proj(l, r, b, t, n, f)
    }
}
