use std::fmt::{Display, Formatter};

pub trait BaseRailRegister {
    fn get_value(&self) -> u8;
    fn set_value(&mut self, value: u8);
    fn set_is_io(&mut self, io: bool);
}

#[derive(Copy)]
#[derive(Clone)]
pub struct RailRegister {
    value: u8,
    is_io: bool
}

impl RailRegister {
    pub fn new() -> Self {
        Self { value: 0, is_io: false }
    }
}

impl BaseRailRegister for RailRegister {
    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
        if self.is_io {
            println!("IO: {}", value);
        }
    }

    fn set_is_io(&mut self, io: bool) {
        self.is_io = io
    }
}

impl Display for RailRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RailRegister: {}", self.get_value())
    }
}
