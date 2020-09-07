#[derive(Debug, PartialEq)]
pub enum Button {
    Start,
    Select,
    A,
    B,
    Left,
    Right,
    Down,
    Up
}

impl Button {
    fn to_bitflag(&self) -> u8 {
        match self {
            A     | Right  => 1,
            B     | Left   => 2,
            Up    | Select => 4,
            Start | Down   => 8,
        }
    }
}

use Button::*;

#[derive(Debug)]
pub struct Input {
    pub dpad: u8,
    pub buttons: u8,
    pub selector: u8,
}

static DPAD_FLAG: u8 = 0x20;
static BUTTONS_FLAG: u8 = 0x10;
static SELECTOR_DEFAULT: u8 = 0xC0;

impl Input {
    pub fn new() -> Self {
        Self {
            dpad: 0x0F,
            buttons: 0x0F,
            selector: SELECTOR_DEFAULT,
        }
    }

    pub fn get(&self) -> u8 {
        if self.selector & DPAD_FLAG > 0 {
            self.dpad | self.selector
        } else if self.selector & BUTTONS_FLAG > 0 {
            self.buttons | self.selector
        } else {
            self.selector | 0x0F
        }
    }

    pub fn set(&mut self, value: u8) {
        self.selector = value | SELECTOR_DEFAULT;
    }

    pub fn key_down(&mut self, button: Button) {
        match button {
            A | B | Select | Start => {
                self.buttons &= !button.to_bitflag();
            }
            _ => self.dpad &= !button.to_bitflag()
        }
    }

    pub fn key_up(&mut self, button: Button) {
        match button {
            A | B | Select | Start => {
                self.buttons |= button.to_bitflag();
            }
            _ => self.dpad |= button.to_bitflag()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut input = Input::new();
        assert_eq!(input.get(), 0b1100_1111);
        input.key_down(Button::Select);
        assert_eq!(input.get(), 0b1100_1111);
        input.set(0b0001_0000);
        assert_eq!(input.get(), BUTTONS_FLAG | 0b1101_1011);
    }
}