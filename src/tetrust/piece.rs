use grid::{Pos, Tile};
use tetrust::color::Color;
use tetrust::shape::Shape;

pub use grid::Grid;

#[derive(Clone, Debug)]
pub struct Piece {
    pub offset: (i32, i32),
    pub orientations: Vec<Vec<Vec<bool>>>,
    pub orientation: usize,
    pub color: Color,
}

impl Piece {
    pub fn new(shape: Shape, color: Color) -> Piece {
        Piece {
            offset: {
                match shape {
                    Shape::O => (4, 0),
                    Shape::I => (3, 0),
                    Shape::T => (4, 0),
                    Shape::S => (4, 0),
                    Shape::Z => (4, 0),
                    Shape::L => (4, 0),
                    Shape::J => (4, 0),
                }
            },
            orientations: {
                match shape {
                    Shape::O => o_template(),
                    Shape::I => i_template(),
                    Shape::T => t_template(),
                    Shape::S => s_template(),
                    Shape::Z => z_template(),
                    Shape::L => l_template(),
                    Shape::J => j_template(),
                }
            },
            orientation: 0,
            color,
        }
    }

    pub fn move_down(&self) -> Self {
        Piece {
            offset: (self.offset.0, self.offset.1 + 1),
            ..self.clone()
        }
    }

    pub fn move_left(&self) -> Self {
        Piece {
            offset: (self.offset.0 - 1, self.offset.1),
            ..self.clone()
        }
    }

    pub fn move_right(&self) -> Self {
        Piece {
            offset: (self.offset.0 + 1, self.offset.1),
            ..self.clone()
        }
    }

    pub fn rotate_right(&self) -> Piece {
        Piece {
            orientation: (self.orientation + 1) % self.orientations.len(),
            ..self.clone()
        }
    }

    pub fn rotate_left(&self) -> Piece {
        Piece {
            orientation: if self.orientation == 0 {
                self.orientations.len() - 1
            } else {
                self.orientation - 1
            },
            ..self.clone()
        }
    }

    pub fn inside_bounds(&self) -> bool {
        let orientation = &self.orientations[self.orientation];
        let width = orientation.len();

        (0..width)
            .flat_map(|y| {
                (0..width).flat_map(move |x| {
                    if orientation[y][x] {
                        Some((self.offset.0 + (x as i32), self.offset.1 + (y as i32)))
                    } else {
                        None
                    }
                })
            })
            .all(|x| inside_bounds(x))
    }
}

impl Grid {
    pub fn draw_piece(&mut self, piece: &Piece, x: u32, y: u32) {
        let orientation = &piece.orientations[piece.orientation];
        let width = orientation.len();

        for j in 0..width {
            for i in 0..width {
                if orientation[j][i] {
                    self[Pos(
                        ((x as i32) + piece.offset.0 + (i as i32)) as u32,
                        ((y as i32) + piece.offset.1 + (j as i32)) as u32,
                    )] = match piece.color {
                        Color::Blue => Tile::BlueSolid,
                        Color::BlueWhite => Tile::BlueWhite,
                        Color::Cyan => Tile::CyanSolid,
                        Color::CyanWhite => Tile::CyanWhite,
                        Color::Grey => Tile::GreySolid,
                        Color::GreyWhite => Tile::GreyWhite,
                    };
                }
            }
        }
    }
}

fn o_template() -> Vec<Vec<Vec<bool>>> {
    vec![vec![vec![true, true], vec![true, true]]]
}

fn i_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![false, false, false, false],
            vec![true, true, true, true],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ],
        vec![
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, false],
        ],
    ]
}

fn t_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
        vec![
            vec![false, true, false],
            vec![false, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, true, false],
        ],
    ]
}

fn s_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![false, true, true],
            vec![true, true, false],
            vec![false, false, false],
        ],
        vec![
            vec![true, false, false],
            vec![true, true, false],
            vec![false, true, false],
        ],
    ]
}

fn z_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![false, false, false],
        ],
        vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![true, false, false],
        ],
    ]
}

fn l_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![true, false, false],
        ],
        vec![
            vec![true, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![false, false, false],
        ],
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, true],
        ],
    ]
}

fn j_template() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![
            vec![true, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
        vec![
            vec![false, true, true],
            vec![false, true, false],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, true],
        ],
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![true, true, false],
        ],
    ]
}

fn inside_bounds(s: (i32, i32)) -> bool {
    s.0 >= 0 && s.0 <= 9 && s.1 >= 0 && s.1 <= 19
}
