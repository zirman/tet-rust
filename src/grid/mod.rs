use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub u32, pub u32);

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
    ExclaimationMark = 26,
    QuotationMark = 27,
    PoundSign = 28,
    DollarSign = 29,
    Percent = 30,
    Period = 31,
    Zero = 32,
    One = 33,
    Two = 34,
    Three = 35,
    Four = 36,
    Five = 37,
    Six = 38,
    Seven = 39,
    Eight = 40,
    Nine = 41,
    Hyphen = 42,
    Asterisk = 43,
    LessThan = 44,
    GreaterThan = 45,
    Underscore = 46,
    Slash = 47,
    Apostrophe = 48,
    OpenParentheses = 49,
    CloseParentheses = 50,
    OpenSquareBrace = 51,
    CloseSquareBrace = 52,
    OpenCurlyBrace = 53,
    CloseCurlyBrace = 54,
    QuestionMark = 55,
    Colon = 56,
    Semicolon = 57,
    Bar = 58,
    Comma = 59,
    Plus = 60,
    EqualSign = 61,
    Block = 62,
    Space = 63,
    GreySolid = 64,
    GreyWhite = 65,
    GreenSolid = 80,
    GreenWhite = 81,
    CyanSolid = 96,
    CyanWhite = 97,
    BlueSolid = 112,
    BlueWhite = 113,
    VioletSolid = 128,
    VioletWhite = 129,
}

#[derive(PartialEq, Debug)]
pub struct Grid {
    width: u32,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let mut tiles = Vec::with_capacity((width * height) as usize);

        for _ in 0..height {
            for _ in 0..width {
                tiles.push(Tile::BlueSolid);
            }
        }

        Grid {
            width: width,
            tiles: tiles,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        (self.tiles.len() / self.width as usize) as u32
    }

    pub fn as_ptr(&self) -> *const Tile {
        self.tiles.as_ptr()
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, t: Tile) {
        for i in x..x + w {
            for j in y..y + h {
                self[Pos(i, j)] = t
            }
        }
    }

    pub fn draw_string(&mut self, p: Pos, str: &str) {
        let mut i = (self.width * p.1 + p.0) as usize;

        for c in str.chars() {
            self.tiles[i] = match c {
                'a' | 'A' => Tile::A,
                'b' | 'B' => Tile::B,
                'c' | 'C' => Tile::C,
                'd' | 'D' => Tile::D,
                'e' | 'E' => Tile::E,
                'f' | 'F' => Tile::F,
                'g' | 'G' => Tile::G,
                'h' | 'H' => Tile::H,
                'i' | 'I' => Tile::I,
                'j' | 'J' => Tile::J,
                'k' | 'K' => Tile::K,
                'l' | 'L' => Tile::L,
                'm' | 'M' => Tile::M,
                'n' | 'N' => Tile::N,
                'o' | 'O' => Tile::O,
                'p' | 'P' => Tile::P,
                'q' | 'Q' => Tile::Q,
                'r' | 'R' => Tile::R,
                's' | 'S' => Tile::S,
                't' | 'T' => Tile::T,
                'u' | 'U' => Tile::U,
                'v' | 'V' => Tile::V,
                'w' | 'W' => Tile::W,
                'x' | 'X' => Tile::X,
                'y' | 'Y' => Tile::Y,
                'z' | 'Z' => Tile::Z,
                '!' => Tile::ExclaimationMark,
                '"' => Tile::QuotationMark,
                '#' => Tile::PoundSign,
                '$' => Tile::DollarSign,
                '%' => Tile::Percent,
                '.' => Tile::Period,
                '0' => Tile::Zero,
                '1' => Tile::One,
                '2' => Tile::Two,
                '3' => Tile::Three,
                '4' => Tile::Four,
                '5' => Tile::Five,
                '6' => Tile::Six,
                '7' => Tile::Seven,
                '8' => Tile::Eight,
                '9' => Tile::Nine,
                '-' => Tile::Hyphen,
                '*' => Tile::Asterisk,
                '<' => Tile::LessThan,
                '>' => Tile::GreaterThan,
                '_' => Tile::Underscore,
                '/' => Tile::Slash,
                '\'' => Tile::Apostrophe,
                '(' => Tile::OpenParentheses,
                ')' => Tile::CloseParentheses,
                '[' => Tile::OpenSquareBrace,
                ']' => Tile::CloseSquareBrace,
                '{' => Tile::OpenCurlyBrace,
                '}' => Tile::CloseCurlyBrace,
                '?' => Tile::QuestionMark,
                ':' => Tile::Colon,
                ';' => Tile::Semicolon,
                '|' => Tile::Bar,
                ',' => Tile::Comma,
                '+' => Tile::Plus,
                '=' => Tile::EqualSign,
                ' ' => Tile::Space,
                _ => Tile::Block,
            };

            i += 1;
        }
    }
}

use std::ops::{Index, IndexMut};

impl Index<Pos> for Grid {
    type Output = Tile;

    fn index(&self, index: Pos) -> &Tile {
        &self.tiles[(self.width * index.1 + index.0) as usize]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut<'a>(&'a mut self, index: Pos) -> &'a mut Tile {
        &mut self.tiles[(self.width * index.1 + index.0) as usize]
    }
}
