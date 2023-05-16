
use crate::rail_system::rail_instruction::RailInstruction;
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
            0 => RailSubSystem::Alu,
            1 => RailSubSystem::RamStack,
            2 => RailSubSystem::CU,
            3 => RailSubSystem::Peripheral,
            _ => RailSubSystem::None
        }
    }

    pub fn get_instruction(&self) -> RailInstruction {
        let masked = self.op & 15;
        match self.get_subsystem() {
            RailSubSystem::Alu => self.get_alu_instruction(masked),
            RailSubSystem::CU => self.get_cu_instruction(masked),
            RailSubSystem::RamStack => self.get_ram_instruction(masked),
            RailSubSystem::Peripheral => self.get_peripheral_instruction(masked),
            _ => RailInstruction::None
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

    pub fn get_alu_instruction(&self, masked: u8) -> RailInstruction {
        match masked {
            0 => RailInstruction::Add,
            1 => RailInstruction::Sub,
            2 => RailInstruction::And,
            3 => RailInstruction::Or,
            4 => RailInstruction::Not,
            5 => RailInstruction::Xor,
            6 => RailInstruction::Shl,
            7 => RailInstruction::Shr,
            // 8, 9, 10, 11
            12 => RailInstruction::RANSetSeed,
            13 => RailInstruction::RANNext,
            14 => RailInstruction::Halt,
            15 => RailInstruction::Noop,
            _ => RailInstruction::None
        }
    }

    pub fn get_cu_instruction(&self, masked: u8) -> RailInstruction {
        match masked {
            0 => RailInstruction::Equals,
            1 => RailInstruction::NotEquals,
            2 => RailInstruction::LessThan,
            3 => RailInstruction::LessEqualThan,
            4 => RailInstruction::MoreThan,
            5 => RailInstruction::MoreEqualThan,
            6 => RailInstruction::True,
            7 => RailInstruction::False,
            // 8 to 15
            _ => RailInstruction::None
        }
    }

    pub fn get_ram_instruction(&self, masked: u8) -> RailInstruction {
        match masked {
            0 => RailInstruction::Read,
            1 => RailInstruction::Write,
            // 2 to 7
            8 => RailInstruction::SPop,
            9 => RailInstruction::SPush,
            10 => RailInstruction::Ret,
            11 => RailInstruction::Call,
            // 12 to 15
            _ => RailInstruction::None
        }
    }

    pub fn get_peripheral_instruction(&self, _masked: u8) -> RailInstruction {
        RailInstruction::None
    }

}
