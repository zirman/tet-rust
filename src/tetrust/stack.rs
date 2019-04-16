use grid::{Grid, Pos, Tile};
use tetrust::color::Color;
use tetrust::piece::Piece;

#[derive(Clone, Debug)]
pub struct Stack {
    pub bricks: [[Option<Color>; 10]; 20],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            bricks: [[None; 10]; 20],
        }
    }

    pub fn overlaps(&self, piece: &Piece) -> bool {
        let orientation = &piece.orientations[piece.orientation];
        let width = orientation.len();

        for y in 0..width {
            for x in 0..width {
                if orientation[y][x] {
                    if self.bricks[(piece.offset.1 + (y as i32)) as usize]
                        [(piece.offset.0 + (x as i32)) as usize]
                        .is_some()
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn remove_lines(&self) -> (Self, u32) {
        let mut bricks = self.bricks;
        let mut lines = 0;
        let mut y = 19;

        loop {
            if bricks[y].iter().all(|x| x.is_some()) {
                lines += 1;

                for j in (1..y).rev() {
                    for x in 0..10 {
                        bricks[j + 1][x] = bricks[j][x];
                    }
                }

                for x in 0..10 {
                    bricks[0][x] = None;
                }

                continue;
            }

            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }

        (Stack { bricks }, lines)
    }

    pub fn can_remove_lines(&self) -> bool {
        let bricks = self.bricks;
        let mut y = 19;

        loop {
            if bricks[y].iter().all(|x| x.is_some()) {
                return true;
            }

            if y == 0 {
                return false;
            } else {
                y -= 1;
            }
        }
    }

    pub fn add_piece(&self, piece: &Piece) -> Self {
        let orientation = &piece.orientations[piece.orientation];
        let width = orientation.len();
        let mut bricks = self.bricks;

        (0..width)
            .flat_map(|y| {
                (0..width).flat_map(move |x| {
                    if orientation[y][x] {
                        Some((
                            (piece.offset.0 + (x as i32)) as usize,
                            (piece.offset.1 + (y as i32)) as usize,
                        ))
                    } else {
                        None
                    }
                })
            })
            .for_each(|(x, y)| {
                bricks[y][x] = Some(piece.color);
            });

        Stack { bricks: bricks }
    }
}

impl Grid {
    pub fn draw_stack(&mut self, stack: &Stack, desolving_rows_timer: u32, x: u32, y: u32) {
        let flash = (desolving_rows_timer / 6) % 2 == 0;

        for j in 0..20 {
            if flash && stack.bricks[j].iter().all(|x| x.is_some()) {
                for i in 0..10 {
                    self[Pos(x + i, y + (j as u32))] = Tile::Space;
                }
            } else {
                for i in 0..10 {
                    self[Pos(x + i, y + (j as u32))] = match stack.bricks[j as usize][i as usize] {
                        Some(color) => match color {
                            Color::Blue => Tile::BlueSolid,
                            Color::BlueWhite => Tile::BlueWhite,
                            Color::Cyan => Tile::CyanSolid,
                            Color::CyanWhite => Tile::CyanWhite,
                            Color::Grey => Tile::GreySolid,
                            Color::GreyWhite => Tile::GreyWhite,
                        },
                        None => Tile::Space,
                    };
                }
            }
        }
    }
}
