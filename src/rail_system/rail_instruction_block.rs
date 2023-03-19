use std::any::Any;
use crate::rail_system::rail_instruction::{NoInstruction, RailALUInstruction, RailCUInstruction, RailPeripheralInstruction, RailRAMInstruction};
use crate::rail_system::rail_subsystem::RailSubSystem;

pub struct RailInstructionBlock {
    pub op: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub result: u8
}

impl RailInstructionBlock {

    pub fn new(op: u8, arg1: u8, arg2: u8, result: u8) -> Self {
        Self { op, arg1, arg2, result }
    }

    pub fn get_subsystem(&self) -> RailSubSystem {
        match (&self.op & 48) >> 4 {
            0 => RailSubSystem::ALU,
            1 => RailSubSystem::RamStack,
            2 => RailSubSystem::CU,
            3 => RailSubSystem::Peripheral,
            _ => RailSubSystem::None
        }
    }

    pub fn get_instruction(&self) -> Box<dyn Any> {
        let masked = self.op & 15;
        match self.get_subsystem() {
            RailSubSystem::ALU => self.get_alu_instruction(masked),
            RailSubSystem::CU => self.get_cu_instruction(masked),
            RailSubSystem::RamStack => self.get_ram_instruction(masked),
            RailSubSystem::Peripheral => self.get_peripheral_instruction(masked),
            _ => Box::new(NoInstruction::None)
        }
    }

    pub fn is_arg1_immediate(&self) -> bool {
        self.check_bit(self.op, 128)
    }

    pub fn is_arg2_immediate(&self) -> bool {
        self.check_bit(self.op, 64)
    }

    pub fn get_ram_target(&self) -> u8 {
        self.result
    }

    pub fn get_result(&self) -> u8 {
        self.result
    }

    pub fn get_cu_addr(&self) -> u8 {
        self.result
    }

    fn check_bit(&self, value: u8, flag: u8) -> bool {
        (value & flag) == flag
    }

    pub fn get_alu_instruction(&self, masked: u8) -> Box<RailALUInstruction> {
        return Box::new(match masked {
            0 => RailALUInstruction::ADD,
            1 => RailALUInstruction::SUB,
            2 => RailALUInstruction::AND,
            3 => RailALUInstruction::OR,
            4 => RailALUInstruction::NOT,
            5 => RailALUInstruction::XOR,
            6 => RailALUInstruction::SHL,
            7 => RailALUInstruction::SHR,
            // 8, 9, 10, 11
            12 => RailALUInstruction::RANSetSeed,
            13 => RailALUInstruction::RANNext,
            // 14
            15 => RailALUInstruction::NOOP,
            _ => RailALUInstruction::None
        })
    }

    pub fn get_cu_instruction(&self, masked: u8) -> Box<RailCUInstruction> {
        Box::new(match masked {
            0 => RailCUInstruction::Equals,
            1 => RailCUInstruction::NotEquals,
            2 => RailCUInstruction::LessThan,
            3 => RailCUInstruction::LessEqualThan,
            4 => RailCUInstruction::MoreThan,
            5 => RailCUInstruction::MoreEqualThan,
            6 => RailCUInstruction::TRUE,
            7 => RailCUInstruction::FALSE,
            // 8 to 15
            _ => RailCUInstruction::None
        })
    }

    pub fn get_ram_instruction(&self, masked: u8) -> Box<RailRAMInstruction> {
        Box::new(match masked {
            0 => RailRAMInstruction::Read,
            1 => RailRAMInstruction::Write,
            // 2 to 7
            8 => RailRAMInstruction::SPop,
            9 => RailRAMInstruction::SPush,
            10 => RailRAMInstruction::Ret,
            11 => RailRAMInstruction::Call,
            // 12 to 15
            _ => RailRAMInstruction::None
        })
    }

    pub fn get_peripheral_instruction(&self, _masked: u8) -> Box<RailPeripheralInstruction> {
        Box::new(RailPeripheralInstruction::None)
    }

}
