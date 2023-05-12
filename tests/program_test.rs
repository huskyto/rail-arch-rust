

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
    fn test_fibonacci() {
        let mut system = RailSystem::new();
        load_asm(&mut system, r#"
                ADD+IM2 R0 1 R1

                LABEL loop
                MOV R2 0 D0
                ADD R1 R2 R2  #this is a comment
                MOV D0 0 R1
                MOV R2 0 IO
                JMP 0 0 loop"#
        );
        system.set_io_print(false);
        for _i in 0..60 {
            system.step();
        }
        assert_eq!(system.get_register_value(2), 0x90);
    }

}
