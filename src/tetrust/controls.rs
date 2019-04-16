#[derive(Clone, Debug)]
pub struct Controls {
    pub up: Option<u32>,
    pub down: Option<u32>,
    pub left: Option<u32>,
    pub right: Option<u32>,
    pub primary: Option<u32>,
    pub secondary: Option<u32>,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            up: None,
            down: None,
            left: None,
            right: None,
            primary: None,
            secondary: None,
        }
    }

    pub fn iterate(&self) -> Self {
        Controls {
            up: self.up.map(|x| x + 1),
            down: self.down.map(|x| x + 1),
            left: self.left.map(|x| x + 1),
            right: self.right.map(|x| x + 1),
            primary: self.primary.map(|x| x + 1),
            secondary: self.secondary.map(|x| x + 1),
        }
    }

    pub fn key_down(&mut self, key_code: u32) {
        match key_code {
            37 => {
                if self.left.is_none() {
                    self.left = Some(0)
                }
            }
            38 => {
                if self.up.is_none() {
                    self.up = Some(0)
                }
            }
            39 => {
                if self.right.is_none() {
                    self.right = Some(0)
                }
            }
            40 => {
                if self.down.is_none() {
                    self.down = Some(0)
                }
            }
            90 => {
                if self.primary.is_none() {
                    self.primary = Some(0)
                }
            }
            88 => {
                if self.secondary.is_none() {
                    self.secondary = Some(0)
                }
            }
            _ => (),
        }
    }

    pub fn key_up(&mut self, key_code: u32) {
        match key_code {
            37 => {
                self.left = None;
                self.right = self.right.map(|_| 1);
            }
            38 => {
                self.up = None;
                self.down = self.down.map(|_| 1);
            }
            39 => {
                self.right = None;
                self.left = self.left.map(|_| 1);
            }
            40 => {
                self.down = None;
                self.up = self.up.map(|_| 1);
            }
            90 => self.primary = None,
            88 => self.secondary = None,
            _ => (),
        }
    }
}
