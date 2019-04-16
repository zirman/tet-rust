use tetrust::controls::Controls;
use tetrust::game::Game;
use tetrust::menu::Menu;
use REGULATOR;

#[derive(Clone, Debug)]
pub struct Regulator {
    t: f32,
    pub controls: Controls,
    pub game: Game,
}

impl Regulator {
    pub fn get(t: f32) -> &'static mut Regulator {
        unsafe {
            REGULATOR.get_or_insert_with(|| Regulator {
                t,
                controls: Controls::new(),
                game: Game::InMenu(Menu::NewGame),
            })
        }
    }

    pub fn iterate(&self, t: f32) -> Self {
        let iterations_per_second = 60.0;
        let d = 1000.0 / iterations_per_second;

        if t - self.t >= 1000.0 {
            self.controls.iterate();
            Regulator {
                t,
                game: self.game.iterate(&self.controls),
                controls: self.controls.iterate(),
            }
        } else if t - self.t >= d {
            Regulator {
                t: self.t + d,
                game: self.game.iterate(&self.controls),
                controls: self.controls.iterate(),
            }
            .iterate(t)
        } else {
            self.clone()
        }
    }
}
