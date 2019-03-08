pub struct Style(pub String);

pub trait Stylable {
    fn to_style(&self) -> Style;
}

pub enum ColorName {
    // HTML 4.01
    White,
    Silver,
    Gray,
    Black,
    Red,
    Maroon,
    Yellow,
    Olive,
    Lime,
    Green,
    Aqua,
    Teal,
    Blue,
    Navy,
    Fuchsia,
    Purple,
    // X11
    Pink,
    Orange,
}

impl Stylable for ColorName {
    fn to_style(&self) -> Style {
        Style(
            match self {
                ColorName::White => "white",
                ColorName::Silver => "silver",
                ColorName::Gray => "gray",
                ColorName::Black => "black",
                ColorName::Red => "red",
                ColorName::Maroon => "maroon",
                ColorName::Yellow => "yellow",
                ColorName::Olive => "olive",
                ColorName::Lime => "lime",
                ColorName::Green => "green",
                ColorName::Aqua => "aqua",
                ColorName::Teal => "teal",
                ColorName::Blue => "blue",
                ColorName::Navy => "navy",
                ColorName::Fuchsia => "fuchsia",
                ColorName::Purple => "purple",
                ColorName::Pink => "pink",
                ColorName::Orange => "orange",
            }.to_string(),
        )
    }
}

pub struct ColorCode {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Stylable for ColorCode {
    fn to_style(&self) -> Style {
        Style(format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        ))
    }
}
