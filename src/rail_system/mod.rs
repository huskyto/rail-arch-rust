use crate::rail_system::rail_instruction::RailInstruction;
use crate::rail_system::rail_instruction_block::RailInstructionBlock;
use crate::rail_system::rail_register::{BaseRailRegister, RailRegister};
use crate::rail_system::rail_subsystem::RailSubSystem;

mod rail_register;
mod rail_instruction;
mod rail_subsystem;
mod rail_instruction_block;

pub struct RailSystem {
    registers: [RailRegister; 16],
    ram: [u8; 256],
    program: [u8; 256],
    call_stack: Vec<u8>
}

pub trait RailSystemTrait {
    fn step(&mut self);
    fn get_register_value(&self, reg: u8) -> u8;
    fn get_cnt_register_value(&self) -> u8;
    fn get_program_slice(&self, start: u8, end: u8) -> &[u8];
    fn get_ram_slice(&self, start: u8, end: u8) -> &[u8];
}

impl RailSystemTrait for RailSystem {

    fn step(&mut self) {
        let instruction = self.get_next_instruction_block();
        self.process_instruction(&instruction);
    }

    fn get_register_value(&self, reg: u8) -> u8 {
        self.registers[reg as usize].get_value()
    }

    fn get_cnt_register_value(&self) -> u8 {
        return self.get_cnt_register().get_value()
    }

    fn get_program_slice(&self, start: u8, end: u8) -> &[u8] {
        &self.program[start as usize .. end as usize]
    }

    fn get_ram_slice(&self, start: u8, end: u8) -> &[u8] {
        return &self.ram[start as usize .. end as usize]
    }
}

impl RailSystem {

    pub fn new() -> Self {
        let mut new_system = Self {
            registers: [RailRegister::new(); 16],
            ram: [0; 256],
            program: [0; 256],
            call_stack: Vec::new()
        };
        new_system.registers[15].set_is_io(true);
        new_system
    }

    pub fn new_with_program(program_slice: &[u8]) -> Self {
        let mut s = Self::new();
        for i in 0..program_slice.len() {
            s.program[i] = program_slice[i];
        }
        return s;
    }

    fn get_cnt_register(&self) -> &RailRegister {
        &self.registers[14]
    }

    fn get_cnt_register_mut(&mut self) -> &mut RailRegister {
        &mut self.registers[14]
    }

    fn get_next_instruction_block(&mut self) -> RailInstructionBlock {
        let program_cnt_reg = &mut self.get_cnt_register_mut();
        let cnt: usize = program_cnt_reg.get_value() as usize;
        program_cnt_reg.set_value(program_cnt_reg.get_value() + 4);

        RailInstructionBlock::new(self.program[cnt],
                                 self.program[cnt + 1],
                                 self.program[cnt + 2],
                                 self.program[cnt + 3])
    }

    fn process_instruction(&mut self, instruction: &RailInstructionBlock) {
        match instruction.get_subsystem() {
            RailSubSystem::ALU => self.process_alu(instruction),
            RailSubSystem::RamStack => self.process_ram_stack(instruction),
            RailSubSystem::CU => self.process_cu(instruction),
            RailSubSystem::Peripheral => { }, // todo
            _ => { }// noop
        }
    }

    fn process_alu(&mut self, instruction: &RailInstructionBlock) {
        let op = instruction.get_instruction();
        let arg1 = self.get_arg1_value(instruction);
        let arg2 = self.get_arg2_value(instruction);
        let mut noop_flag = false;
        let res = match op {
            RailInstruction::Add => arg1.wrapping_add(arg2),
            RailInstruction::Sub => arg1 - arg2,
            RailInstruction::And => arg1 & arg2,
            RailInstruction::Or => arg1 | arg2,
            RailInstruction::Not => !arg1,
            RailInstruction::Xor => arg1 ^ arg2,
            RailInstruction::Shl => arg1 << arg2,
            RailInstruction::Shr => arg1 >> arg2,
            RailInstruction::RANSetSeed => { 0 },  //TODO(),
            RailInstruction::RANNext => { 0 },  //TODO(),
            RailInstruction::Noop => {
                noop_flag = true; 0 // noop
            }
            _ => {
                noop_flag = true; 0 // noop
            }
        };

        if noop_flag {
            return;
        }

        let res_reg = &mut self.registers[instruction.get_result() as usize];
        res_reg.set_value(res);
    }

    fn process_ram_stack(&mut self, instruction: &RailInstructionBlock) {
        let op = instruction.get_instruction();
        let source = self.get_arg1_value(instruction);
        let addr = self.get_arg2_value(instruction) as usize;
        let target = instruction.get_ram_target();
        let target_reg = &mut self.registers[target as usize];
        match op {
            RailInstruction::Read  => target_reg.set_value(self.ram[addr]),
            RailInstruction::Write => self.ram[addr] = self.registers[source as usize].get_value(),
            RailInstruction::SPop  => { },//TODO(),
            RailInstruction::SPush => { },//TODO(),
            RailInstruction::Ret => {
                let cnt = self.call_stack.pop().unwrap();
                self.get_cnt_register_mut().set_value(cnt)
            },
            RailInstruction::Call => {
                self.call_stack.push(self.get_cnt_register_value());  //already moved to next in step
                self.get_cnt_register_mut().set_value(source)
            },
            RailInstruction::None => {} //noop
            _ => { }
        };
    }

    fn process_cu(&mut self, instruction: &RailInstructionBlock) {
        let op = instruction.get_instruction();
        let arg1 = self.get_arg1_value(instruction);
        let arg2 = self.get_arg2_value(instruction);
        let jmp_addr = instruction.get_cu_addr();   // always immediate

        let do_jmp = match op {
            RailInstruction::Equals => arg1 == arg2,
            RailInstruction::NotEquals => arg1 != arg2,
            RailInstruction::LessThan => arg1 < arg2,
            RailInstruction::LessEqualThan => arg1 <= arg2,
            RailInstruction::MoreThan => arg1 > arg2,
            RailInstruction::MoreEqualThan => arg1 >= arg2,
            RailInstruction::True => true,
            RailInstruction::False => false,
            _ => false
        };
        if do_jmp {
            self.get_cnt_register_mut().set_value(jmp_addr);
        }
    }

    fn get_arg1_value(&self, instruction: &RailInstructionBlock) -> u8 {
        if instruction.is_arg1_immediate() {
            instruction.arg1
        }
        else {
            self.registers[instruction.arg1 as usize].get_value()
        }
    }

    fn get_arg2_value(&self, instruction: &RailInstructionBlock) -> u8 {
        if instruction.is_arg2_immediate() {
            instruction.arg2
        }
        else {
            self.registers [instruction.arg2 as usize].get_value()
        }
    }

}
