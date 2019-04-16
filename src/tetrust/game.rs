use grid::{Pos, Tile};
use rand::{rngs::SmallRng, SeedableRng};
use tetrust::controls::Controls;
use tetrust::menu::Menu;
use tetrust::piece::{Grid, Piece};
use tetrust::playing::{Level, Mode, Playing};
use GRID_HEIGHT;
use GRID_WIDTH;

#[derive(Clone, Debug)]
pub enum Game {
    InMenu(Menu),
    Playing(Playing),
}

pub fn input_trigger(
    input: Option<u32>,
    opposing_input: Option<u32>,
    start: u32,
    interval: u32,
) -> bool {
    match input {
        Some(x) => {
            if x < start {
                x == 0
            } else if opposing_input.is_none() {
                (x - start) % interval == 0
            } else {
                false
            }
        }
        None => false,
    }
}

impl Game {
    pub fn iterate(&self, controls: &Controls) -> Self {
        match self {
            Game::InMenu(menu) => match menu {
                Menu::NewGame => {
                    if controls.primary.is_some() {
                        let mut rng =
                            SmallRng::from_seed([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

                        Game::Playing(Playing::new(Level::new(0), &rng))
                    } else if input_trigger(controls.down, controls.up, 30, 10)
                        || input_trigger(controls.up, controls.down, 30, 10)
                    {
                        Game::InMenu(Menu::Options)
                    } else {
                        Game::InMenu(Menu::NewGame)
                    }
                }

                Menu::Options => {
                    if input_trigger(controls.up, controls.down, 30, 10)
                        || input_trigger(controls.down, controls.up, 30, 10)
                    {
                        Game::InMenu(Menu::NewGame)
                    } else {
                        Game::InMenu(Menu::Options)
                    }
                }
            },
            Game::Playing(playing) => Game::Playing(playing.iterate(&controls)),
        }
    }

    pub fn grid(&self) -> Grid {
        let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

        match self {
            Game::InMenu(menu) => {
                grid.draw_rect(14, 1, 20, GRID_HEIGHT - 2, Tile::Space);
                grid.draw_string(Pos(21, 3), "Tet-Rust");
                grid.draw_string(Pos(19, 7), "Start Game");
                grid.draw_string(Pos(19, 9), "Options");
                match menu {
                    Menu::NewGame => grid[Pos(18, 7)] = Tile::Asterisk,
                    Menu::Options => grid[Pos(18, 9)] = Tile::Asterisk,
                }
            }
            Game::Playing(playing) => {
                grid.draw_rect(25, 4, 9, 6, Tile::Space);
                grid.draw_rect(25, 11, 9, 6, Tile::Space);
                grid.draw_rect(25, 18, 9, 6, Tile::Space);
                grid.draw_rect(14, 1, 20, 2, Tile::Space);

                grid.draw_string(Pos(21, 1), "Tet-Rust");

                match &playing.mode {
                    Mode::DissolvingRows(timer) => {
                        grid.draw_stack(&playing.stack, *timer, 14, 4);
                    }
                    Mode::DroppingPiece(piece) => {
                        grid.draw_stack(&playing.stack, 0, 14, 4);
                        grid.draw_piece(piece, 14, 4);
                    }
                }

                grid.draw_string(Pos(27, 4), "Next");
                grid.draw_piece(&Piece::new(playing.shape, playing.color), 24, 6);

                grid.draw_string(Pos(25, 18), &format!("{}", playing.time_elapsed));
                grid.draw_string(Pos(25, 19), &format!("{}", playing.pieces_dropped));
                grid.draw_string(Pos(25, 20), &format!("{}", playing.rows_removed));
                grid.draw_string(Pos(25, 21), &format!("{}", playing.score));
                grid.draw_string(
                    Pos(25, 22),
                    &format!(
                        "{}",
                        playing.base_level.adjusted_level(playing.rows_removed)
                    ),
                );

                if playing.game_over {
                    grid.draw_string(Pos(15, 13), "GAME OVER");
                }
            }
        }
        grid
    }
}
