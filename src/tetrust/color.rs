use rand::{rngs::SmallRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Blue,
    BlueWhite,
    Cyan,
    CyanWhite,
    Grey,
    GreyWhite,
}

impl Color {
    pub fn rand(rng: &mut SmallRng) -> Self {
        match rng.gen_range(0, 6) {
            0 => Color::Blue,
            1 => Color::BlueWhite,
            2 => Color::Cyan,
            3 => Color::CyanWhite,
            4 => Color::Grey,
            _ => Color::GreyWhite,
        }
    }
}
