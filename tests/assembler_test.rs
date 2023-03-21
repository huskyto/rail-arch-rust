
#[path = "../src/rail_assembler/mod.rs"]
pub mod rail_assembler;

pub use crate::rail_assembler::rasm_dictionary::RasmDictionary;

#[cfg(test)]
mod tests {
    pub use crate::rail_assembler::{RailAssembler, RailAssemblerTrait};
    pub use crate::rail_assembler::rasm_dictionary::RasmDictionary;

    fn assemble(asm: &str) -> Vec<u8> {
        let rail_assembler = RailAssembler::new();
        rail_assembler.assemble(asm)
    }

    #[test]
    fn test_num_encode_dec() {
        let assembled = assemble("ADD 8 12 14");
        assert_eq!(assembled, &[0x00, 8, 12, 14]);
    }

    #[test]
    fn test_num_encode_hex() {
        let assembled = assemble("ADD 0x08 0x12 0x14");
        assert_eq!(assembled, &[0x00, 0x08, 0x12, 0x14]);
    }

    #[test]
    fn test_num_encode_oct() {
        let assembled = assemble("ADD 0o07 0o12 0o14");
        assert_eq!(assembled, &[0x00, 0o07, 0o12, 0o14]);
    }

    #[test]
    fn test_num_encode_bin() {
        let assembled = assemble("ADD 0b0101 0b1010 0b1001");
        assert_eq!(assembled, &[0x00, 0b0101, 0b1010, 0b1001]);
    }

}