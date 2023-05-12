
#[path = "../src/rail_system/mod.rs"]
pub mod rail_system;
#[path = "../src/rail_assembler/mod.rs"]
pub mod rail_assembler;

#[cfg(test)]
mod tests {
    pub use crate::rail_system::{RailSystem, RailSystemTrait};
    pub use crate::rail_assembler::{RailAssembler, RailAssemblerTrait};

    fn load_asm(rs: &mut RailSystem, asm: &str) {
        let rail_assembler = RailAssembler::new();
        let assembled = rail_assembler.assemble(asm);
        rs.load_program(&assembled[..]);
    }

    #[test]
    fn test_get_register_value() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 "ADD+IM2 0 1 R1"
        );
        system.step();
        assert_eq!(system.get_register_value(1), 0x01);
    }

    #[test]
    fn test_get_cnt_register_value() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 r#"NOOP 0 0 0
                NOOP 0 0 0"#
        );
        system.step();
        system.step();
        assert_eq!(system.get_cnt_register_value(), 0x08);
    }

    #[test]
    fn test_get_program_slice() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 "ADD+IM2 0 1 R1"
        );
        assert_eq!(system.get_program_slice(0, 3), &[0x40, 0x00, 0x01, 0x01]);
    }

    #[test]
    fn test_get_ram_slice() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 r#"
                 RAM_W+IM1+IM2 0 0 0
                 ADD+IM2 0 1 R1
                 RAM_W+IM1+IM2 1 1 0
                 ADD+IM2 0 2 R2
                 RAM_W+IM1+IM2 2 2 0
                 ADD+IM2 0 3 R3
                 RAM_W+IM1+IM2 3 3 0"#
        );
        system.step();
        system.step();
        system.step();
        system.step();
        system.step();
        system.step();
        system.step();
        system.step();
        assert_eq!(system.get_ram_slice(0, 5), &[0x00, 0x01, 0x02, 0x03, 0x00, 0x00]);
    }

    #[test]
    fn test_set_io_print() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 "ADD+IM2 0 1 IO"
        );
        system.step();
        system.set_io_print(true);
    }

    #[test]
    fn test_xor_shift_rng() {
        let mut system = RailSystem::new();
        load_asm(&mut system,
                 r#"RAN_SS+IM1 29 0 0
                        RAN_NEXT 0 0 R1
                        JMP 0 0 4"#
        );
        let mut outs: Vec<u8> = Vec::new();
        system.step();
        for _ in 0..8 {
            system.step();
            outs.push(system.get_register_value(1));
            system.step();
        }

        assert_eq!(outs, &[56, 119, 225, 159, 108, 213, 241, 189]);
    }

}
