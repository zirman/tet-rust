use rand::rngs::SmallRng;
use std::fmt::{Display, Formatter, Result};
use tetrust::color::Color;
use tetrust::controls::Controls;
use tetrust::piece::Piece;
use tetrust::shape::Shape;
use tetrust::stack::Stack;

#[derive(Clone, Copy, Debug)]
pub struct Level {
    num: u32,
}

impl Level {
    pub fn new(num: u32) -> Self {
        Level { num }
    }
    pub fn adjusted_level(self, rows_removed: RowsRemoved) -> Level {
        Level {
            num: self.num + rows_removed.num / 10,
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LVL:{}", self.num)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Score {
    num: u32,
}

impl Score {
    fn new() -> Self {
        Score { num: 0 }
    }
    fn add(self, points: u32) -> Self {
        Score {
            num: self.num + points,
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PTS:{}", self.num)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimeElapsed {
    num: u32,
}

impl TimeElapsed {
    fn new() -> Self {
        TimeElapsed { num: 0 }
    }
    fn inc(self) -> Self {
        TimeElapsed { num: self.num + 1 }
    }
}

impl Display for TimeElapsed {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TIM:{}", self.num / 60)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RowsRemoved {
    num: u32,
}

impl RowsRemoved {
    fn new() -> Self {
        RowsRemoved { num: 0 }
    }
    fn add(self, num: u32) -> Self {
        RowsRemoved {
            num: self.num + num,
        }
    }
}

impl Display for RowsRemoved {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ROW:{}", self.num)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PiecesDropped {
    num: u32,
}

impl PiecesDropped {
    fn new() -> Self {
        PiecesDropped { num: 0 }
    }
    fn inc(self) -> Self {
        PiecesDropped { num: self.num + 1 }
    }
}

impl Display for PiecesDropped {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PCS:{}", self.num)
    }
}

#[derive(Clone, Debug)]
pub enum Mode {
    DroppingPiece(Piece),
    DissolvingRows(u32),
    //    Paused
}

#[derive(Clone, Debug)]
pub struct Playing {
    pub stack: Stack,
    pub shape: Shape,
    pub color: Color,
    pub mode: Mode,
    pub rng: SmallRng,
    pub base_level: Level,
    pub rows_removed: RowsRemoved,
    pub score: Score,
    pub pieces_dropped: PiecesDropped,
    pub time_elapsed: TimeElapsed,
    pub game_over: bool,
}

use tetrust::game::input_trigger;

impl Playing {
    pub fn new(base_level: Level, rng: &SmallRng) -> Self {
        let mut rng = rng.clone();

        Playing {
            game_over: false,
            stack: Stack::new(),
            shape: Shape::rand(&mut rng),
            color: Color::rand(&mut rng),
            mode: Mode::DroppingPiece(Piece::new(Shape::rand(&mut rng), Color::rand(&mut rng))),
            base_level: base_level,
            score: Score::new(),
            time_elapsed: TimeElapsed::new(),
            rng: rng,
            pieces_dropped: PiecesDropped::new(),
            rows_removed: RowsRemoved::new(),
        }
    }

    fn increment_time(&self) -> Self {
        Playing {
            time_elapsed: self.time_elapsed.inc(),
            ..self.clone()
        }
    }

    fn dissolve_rows(&self, controls: &Controls) -> Self {
        match &self.mode {
            Mode::DroppingPiece(piece) => {
                match piece
                    .input_move_left(&controls, &self.stack)
                    .input_move_right(&controls, &self.stack)
                    .input_rotate_left(&controls, &self.stack)
                    .input_rotate_right(&controls, &self.stack)
                    .input_move_down(&controls, &self.stack)
                    .and_then(|piece| {
                        piece.automatically_move_down(
                            self.time_elapsed,
                            self.base_level,
                            self.rows_removed,
                            &self.stack,
                        )
                    }) {
                    Ok(piece) => Playing {
                        mode: Mode::DroppingPiece(piece),
                        ..self.clone()
                    },
                    Err(stack) => Playing {
                        mode: Mode::DissolvingRows(if stack.can_remove_lines() { 30 } else { 15 }),
                        stack: stack,
                        pieces_dropped: self.pieces_dropped.inc(),
                        ..self.clone()
                    },
                }
            }
            Mode::DissolvingRows(1) => {
                let mut rng = self.rng.clone();
                let piece = Piece::new(self.shape, self.color);
                let (stack, num_rows) = self.stack.remove_lines();

                Playing {
                    game_over: stack.overlaps(&piece),
                    shape: Shape::rand(&mut rng),
                    color: Color::rand(&mut rng),
                    rng: rng,
                    score: self.score.add(
                        num_rows
                            * (100 + 10 * self.base_level.adjusted_level(self.rows_removed).num),
                    ),
                    rows_removed: self.rows_removed.add(num_rows),
                    mode: Mode::DroppingPiece(piece),
                    stack: stack,
                    ..self.clone()
                }
            }
            Mode::DissolvingRows(timer) => Playing {
                mode: Mode::DissolvingRows(timer - 1),
                ..self.clone()
            },
        }
    }

    pub fn iterate(&self, controls: &Controls) -> Self {
        if self.game_over {
            return self.clone();
        }

        self.increment_time().dissolve_rows(&controls)
    }
}

use std::result::Result as Res;

impl Piece {
    fn input_move_left(&self, controls: &Controls, stack: &Stack) -> Self {
        self.try_moving_piece(stack, controls.left, controls.right, |piece| {
            piece.move_left()
        })
    }

    fn input_move_right(&self, controls: &Controls, stack: &Stack) -> Self {
        self.try_moving_piece(stack, controls.right, controls.left, |piece| {
            piece.move_right()
        })
    }

    fn input_rotate_left(&self, controls: &Controls, stack: &Stack) -> Self {
        self.try_moving_piece(stack, controls.primary, controls.secondary, |piece| {
            piece.rotate_left()
        })
    }

    fn input_rotate_right(&self, controls: &Controls, stack: &Stack) -> Self {
        self.try_moving_piece(stack, controls.secondary, controls.primary, |piece| {
            piece.rotate_right()
        })
    }

    fn input_move_down(&self, controls: &Controls, stack: &Stack) -> Res<Self, Stack> {
        if input_trigger(controls.down, controls.up, 20, 1) {
            self.try_move_down(stack)
        } else {
            Ok(self.clone())
        }
    }

    fn automatically_move_down(
        &self,
        time_elapsed: TimeElapsed,
        base_level: Level,
        rows_removed: RowsRemoved,
        stack: &Stack,
    ) -> Res<Self, Stack> {
        if time_elapsed.num % (30 - 2 * base_level.adjusted_level(rows_removed).num) == 0 {
            self.try_move_down(stack)
        } else {
            Ok(self.clone())
        }
    }

    fn try_moving_piece<F: FnOnce(&Piece) -> Piece>(
        &self,
        stack: &Stack,
        dir: Option<u32>,
        other_dir: Option<u32>,
        m: F,
    ) -> Self {
        if input_trigger(dir, other_dir, 20, 5) {
            let moved_piece = m(self);

            if moved_piece.inside_bounds() && !stack.overlaps(&moved_piece) {
                moved_piece
            } else {
                self.clone()
            }
        } else {
            self.clone()
        }
    }

    fn try_move_down(&self, stack: &Stack) -> Res<Self, Stack> {
        let moved_piece = self.move_down();

        if !moved_piece.inside_bounds() || stack.overlaps(&moved_piece) {
            Err(stack.add_piece(self))
        } else {
            Ok(moved_piece)
        }
    }
}
