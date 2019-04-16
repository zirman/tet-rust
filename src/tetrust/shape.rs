use rand::{rngs::SmallRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    O,
    I,
    T,
    S,
    Z,
    L,
    J,
}

impl Shape {
    pub fn rand(rng: &mut SmallRng) -> Self {
        match rng.gen_range(0, 7) {
            0 => Shape::O,
            1 => Shape::I,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::L,
            _ => Shape::J,
        }
    }
}
