
pub enum RailInstruction {
    //ALU
    Add, Sub, And, Or, Not, Xor, Shl, Shr, RANSetSeed, RANNext, Noop,
    // CU
    Equals, NotEquals, LessThan, LessEqualThan, MoreThan, MoreEqualThan, True, False,
    // RAM
    Read, Write, SPop, SPush, Ret, Call,
    // Peripheral

    // None
    None
}
