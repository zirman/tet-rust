use super::{M4, V3};

pub struct Quaternion {
    arr: [f32; 4],
}

impl Quaternion {
    pub fn x(&self) -> f32 {
        self.arr[0]
    }
    pub fn y(&self) -> f32 {
        self.arr[1]
    }
    pub fn z(&self) -> f32 {
        self.arr[2]
    }
    pub fn w(&self) -> f32 {
        self.arr[3]
    }
    pub fn identity() -> Self {
        Quaternion {
            arr: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn rotation(axis: &V3, angle: f32) -> Self {
        Quaternion {
            arr: [
                axis.x() * (angle * 0.5).sin(),
                axis.y() * (angle * 0.5).sin(),
                axis.z() * (angle * 0.5).sin(),
                (angle * 0.5).cos(),
            ],
        }
    }
    pub fn to_matrix(&self) -> M4 {
        let xx = self.x() * self.x();
        let yy = self.y() * self.y();
        let zz = self.z() * self.z();
        let ww = self.w() * self.w();
        let xy = self.x() * self.y();
        let yz = self.y() * self.z();
        let zw = self.z() * self.w();
        let xw = self.x() * self.w();
        let xz = self.x() * self.z();
        let yw = self.y() * self.w();

        M4::new(
            xx + yy + zz + ww,
            2.0 * (xy - zw),
            2.0 * (xz + yw),
            0.0,
            2.0 * (xy + zw),
            ww - xx + yy - zz,
            2.0 * (yz + xw),
            0.0,
            2.0 * (xz - yw),
            2.0 * (yz - xw),
            ww - xx - yy + zz,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}
