pub struct V2 {
    arr: [f32; 2],
}

impl V2 {
    pub fn new(x: f32, y: f32) -> Self {
        V2 { arr: [x, y] }
    }
    pub fn x(&self) -> f32 {
        self.arr[0]
    }
    pub fn y(&self) -> f32 {
        self.arr[1]
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
    pub fn neg(&self) -> Self {
        V2::new(-self.x(), -self.y())
    }
    pub fn add(&self, rhs: &Self) -> Self {
        V2::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        V2::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
    pub fn mul(&self, s: f32) -> Self {
        V2::new(self.x() * s, self.y() * s)
    }
    pub fn sqrd_len(&self) -> f32 {
        self.dot(self)
    }
}

pub struct V3 {
    arr: [f32; 3],
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        V3 { arr: [x, y, z] }
    }
    pub fn x(&self) -> f32 {
        self.arr[0]
    }
    pub fn y(&self) -> f32 {
        self.arr[1]
    }
    pub fn z(&self) -> f32 {
        self.arr[2]
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * self.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    pub fn neg(&self) -> Self {
        V3::new(-self.x(), -self.y(), -self.z())
    }
    pub fn add(&self, rhs: &Self) -> Self {
        V3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        V3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
    pub fn mul(&self, s: f32) -> Self {
        V3::new(self.x() * s, self.y() * s, self.z() * s)
    }
    pub fn sqrd_len(&self) -> f32 {
        self.dot(self)
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        V3::new(
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        )
    }
    pub fn to_v4(&self) -> V4 {
        V4::new(self.x(), self.y(), self.z(), 1.0)
    }
}

pub struct V4 {
    arr: [f32; 4],
}

impl V4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        V4 { arr: [x, y, z, w] }
    }
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
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() * rhs.w()
    }
    pub fn neg(&self) -> Self {
        V4::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
    pub fn add(&self, rhs: &Self) -> Self {
        V4::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            self.w() + rhs.w(),
        )
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        V4::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            self.w() - rhs.w(),
        )
    }
    pub fn mul(&self, s: f32) -> Self {
        V4::new(self.x() * s, self.y() * s, self.z() * s, self.w() * s)
    }
    pub fn sqrd_len(&self) -> f32 {
        self.dot(self)
    }
    pub fn to_v2(&self) -> V2 {
        V2::new(self.x() / self.w(), self.y() / self.w())
    }
    pub fn norm(&self) -> Self {
        V4::new(
            self.x() / self.w(),
            self.y() / self.w(),
            self.y() / self.w(),
            1.0,
        )
    }
}
