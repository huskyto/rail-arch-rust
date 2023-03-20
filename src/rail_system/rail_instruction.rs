
pub enum RailInstruction {
    //ALU
    ADD, SUB, AND, OR, NOT, XOR, SHL, SHR, RANSetSeed, RANNext, NOOP,
    // CU
    Equals, NotEquals, LessThan, LessEqualThan, MoreThan, MoreEqualThan, TRUE, FALSE,
    // RAM
    Read, Write, SPop, SPush, Ret, Call,
    // Peripheral

    // None
    None
}
