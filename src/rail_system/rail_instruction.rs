use std::any::Any;

pub trait RailInstruction: Any { }

impl RailInstruction for RailALUInstruction { }
impl RailInstruction for RailCUInstruction { }
impl RailInstruction for RailRAMInstruction { }
impl RailInstruction for RailPeripheralInstruction { }
impl RailInstruction for NoInstruction { }

impl dyn RailInstruction { }

pub enum RailALUInstruction  {
    ADD, SUB, AND, OR, NOT, XOR, SHL, SHR, RANSetSeed, RANNext, NOOP, None
}

pub enum RailCUInstruction  {
    Equals, NotEquals, LessThan, LessEqualThan, MoreThan, MoreEqualThan, TRUE, FALSE, None
}
pub enum RailRAMInstruction {
    Read, Write, SPop, SPush, Ret, Call, None
}

pub enum RailPeripheralInstruction {
    None
}

pub enum NoInstruction {
    None
}