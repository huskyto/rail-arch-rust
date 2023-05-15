
pub struct RasmDictionary { }

impl RasmDictionary {

    pub fn translate(token: &str) -> Result<u8, String> {
        let value = match token {
                // REGISTERS //
            "R0" => 0x00,
            "R1" => 0x01,
            "R2" => 0x02,
            "R3" => 0x03,
            "R4" => 0x04,
            "R5" => 0x05,
            "R6" => 0x06,
            "R7" => 0x07,

            "BZ0" => 0x08,
            "LV0" => 0x09,
            "D0" => 0x0A,
            "D1" => 0x0B,
            "D2" => 0x0C,
            "D3" => 0x0D,
            "CNT" => 0x0E,
            "IO" => 0x0F,

                // ALU //
            "ADD" => 0x00,
            "SUB" => 0x01,
            "AND" => 0x02,
            "OR" => 0x03,
            "NOT" => 0x04,
            "XOR" => 0x05,
            "SHL" => 0x06,
            "SHR" => 0x07,

            "RAN_SS" => 0x0C,
            "RAN_NEXT" => 0x0D,

            "NOOP" => 0x0F,

                // CU //
            "IF_EQ" => 0x20,
            "IF_N_EQ" => 0x21,
            "IF_LT" => 0x22,
            "IF_LTE" => 0x23,
            "IF_MT" => 0x24,
            "IF_MTE" => 0x25,
            "IF_T" => 0x26,  // check exact meaning
            "IF_F" => 0x27,  // check exact meaning

                // RAM_STACK //
            "RAM_R" => 0x10,
            "RAM_W" => 0x11,

            "S_POP" => 0x18,
            "S_PUSH" => 0x19,
            "RET" => 0x1A,
            "CALL" => 0x9B,   //0x1B  IM included

                // IMMEDIATE //
            "IM2" => 0x40,
            "IM1" => 0x80,

                // ALIAS //
            "MOV" => 0x40,   //0x00,
            "JMP" => 0x26,

            _ => 0xFF   // unnecessary error code
        };
        if value == 0xFF {
            Err("Not in dictionary".to_string())
        } else {
            Ok(value)
        }
    }

}