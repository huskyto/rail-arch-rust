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
    call_stack: [u8; 128],
    call_stack_ptr: u8,
    gen_stack: [u8; 128],
    gen_stack_ptr: u8,
    ran_seed: u8,

    is_halted: bool
}

pub trait RailSystemTrait {
    fn step(&mut self);
    fn get_register_value(&self, reg: u8) -> u8;
    fn get_cnt_register_value(&self) -> u8;
    fn get_program_slice(&self, start: u8, end: u8) -> &[u8];
    fn get_ram_slice(&self, start: u8, end: u8) -> &[u8];
    fn get_call_stack_slice(&self, start: u8, end: u8) -> &[u8];
    fn get_call_stack_ptr(&self) -> u8;
    fn get_gen_stack_slice(&self, start: u8, end: u8) -> &[u8];
    fn get_gen_stack_ptr(&self) -> u8;
    fn is_halted(&self) -> bool;
    fn set_io_print(&mut self, print: bool);

    fn load_program(&mut self, program_slice: &[u8]);
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
        &self.program[start as usize ..=end as usize]
    }

    fn get_ram_slice(&self, start: u8, end: u8) -> &[u8] {
        &self.ram[start as usize ..=end as usize]
    }

    fn get_call_stack_slice(&self, start: u8, end: u8) -> &[u8] {
        &self.call_stack[start as usize ..=end as usize]
    }

    fn get_call_stack_ptr(&self) -> u8 {
        self.call_stack_ptr
    }

    fn get_gen_stack_slice(&self, start: u8, end: u8) -> &[u8] {
        &self.gen_stack[start as usize ..=end as usize]
    }

    fn get_gen_stack_ptr(&self) -> u8 {
        self.gen_stack_ptr
    }

    fn set_io_print(&mut self, print: bool) {
        self.registers[15].set_is_io(print);
    }

    fn load_program(&mut self, program_slice: &[u8]) {
        self.program[..program_slice.len()].copy_from_slice(program_slice);
    }

    fn is_halted(&self) -> bool {
        self.is_halted
    }

}

impl RailSystem {

    pub fn new() -> Self {
        let mut new_system = Self {
            registers: [RailRegister::new(); 16],
            ram: [0; 256],
            program: [0; 256],
            call_stack: [0; 128],
            call_stack_ptr: 0xFF,
            gen_stack: [0; 128],
            gen_stack_ptr: 0xFF,
            ran_seed: 0,
            is_halted: false
        };
        new_system.registers[15].set_is_io(true);
        new_system
    }

    pub fn new_with_program(program_slice: &[u8]) -> Self {
        let mut new_system = Self::new();
        new_system.load_program(program_slice);
        new_system
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
            RailSubSystem::Alu => self.process_alu(instruction),
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
            RailInstruction::Sub => arg1.wrapping_sub(arg2),
            RailInstruction::And => arg1 & arg2,
            RailInstruction::Or => arg1 | arg2,
            RailInstruction::Not => !arg1,
            RailInstruction::Xor => arg1 ^ arg2,
            RailInstruction::Shl => arg1 << arg2,
            RailInstruction::Shr => arg1 >> arg2,
            RailInstruction::RANSetSeed => {
                self.ran_set_seed(arg1);
                noop_flag = true; 0 // should not update anything
            },
            RailInstruction::RANNext => self.ran_next(),
            RailInstruction::Halt => {
                self.is_halted = true; 0
            }
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

    fn ran_set_seed(&mut self, seed: u8) {
        self.ran_seed = seed;
    }

    fn ran_next(&mut self) -> u8 {
        let tmp1 = self.ran_seed ^ (self.ran_seed >> 1);
        let tmp2 = tmp1 ^ (tmp1 << 1);
        self.ran_seed = tmp2 ^ (tmp2 >> 2);
        self.ran_seed
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
            RailInstruction::SPop  => {
                let value = self.pop_gen_stack();
                self.registers[target as usize].set_value(value)
            },
            RailInstruction::SPush => {
                let value = self.registers[source as usize].get_value();
                self.push_gen_stack(value);
            },
            RailInstruction::Ret => {
                let cnt = self.pop_call_stack();
                self.get_cnt_register_mut().set_value(cnt)
            },
            RailInstruction::Call => {
                self.push_call_stack(self.get_cnt_register_value());  //already moved to next in step
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

    fn push_call_stack(&mut self, value: u8) {
        self.call_stack_ptr = self.call_stack_ptr.wrapping_add(1);
        self.call_stack[self.call_stack_ptr as usize] = value;
    }

    fn pop_call_stack(&mut self) -> u8 {
        let res = self.call_stack[self.call_stack_ptr as usize];
        self.call_stack_ptr = self.call_stack_ptr.wrapping_sub(1);
        res
    }

    fn push_gen_stack(&mut self, value: u8) {
        self.gen_stack_ptr = self.gen_stack_ptr.wrapping_add(1);
        self.gen_stack[self.gen_stack_ptr as usize] = value;
    }

    fn pop_gen_stack(&mut self) -> u8 {
        let res = self.gen_stack[self.gen_stack_ptr as usize];
        self.gen_stack_ptr = self.gen_stack_ptr.wrapping_sub(1);
        res
    }

}
