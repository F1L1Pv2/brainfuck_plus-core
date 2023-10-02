pub const MEM_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident,        // names
    MacroDecl,    // #define
    IfdefMacro,   // #ifdef
    IfNdefMacro,  // #ifndef
    ElseMacro,    // #else macro
    EndifMacro,   // #endif
    IncludeMacro, // #include
    PointerLeft,  // <
    PointerRight, // >
    PointerReset, // &
    Add,          // +
    Sub,          // -
    ReadByte,     // ,
    WriteByte,    // .
    Clear,        // '
    BaseMemAddr,  // %
    MemAddr,      // $
    ZeroJump,     // [
    NonZeroJump,  // ]
    Syscall,      // ?
    NewLine,      // \n
    Push,         // ^
    Pop,          // _
    IntLit,       // 0123
    StringLit,    // "baller"
    IncludePath,  // (std/loops.bf)
    TapeDecl,     // #tape
    CellSize,     // byte, word, dword, qword
    TapeName,     // `{main}`
    CurrentTape,  // @`{TapeName}`
    StackDel,     // *
    BitwiseLeft, // ;
    BitwiseRight, // :
    BitwiseAnd, // \
    BitwiseOr, // |
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Dword,
    Qword,
}

#[derive(Debug, Clone)]
pub struct Tape {
    pub name: String,
    pub size: Size,
    pub cell_count: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub row: usize,
    pub col: usize,
    pub filename: String
}

#[derive(PartialEq, Debug)]
pub struct Condition {
    pub addr: usize,
}

#[derive(PartialEq, Debug)]
pub struct Forward {
    pub back_addr: usize,
}

#[derive(PartialEq, Debug)]
pub enum Jumps {
    Condition(Condition),
    Forward(Forward),
}

#[derive(Debug)]
pub struct Operation {
    pub token_type: TokenType,
    pub count: usize,
    pub values: Vec<String>,
    pub tape: Option<Tape>,
    pub row: usize,
    pub col: usize,
    pub filename: String
}

pub fn cross_reference(operations: &[Operation]) -> Vec<Jumps> {
    let mut jumps: Vec<Jumps> = Vec::new();

    for operation in operations {
        match operation.token_type {
            TokenType::ZeroJump => {
                jumps.push(Jumps::Condition(Condition { addr: 0 }));
            }
            TokenType::NonZeroJump => {
                let len = jumps.len();

                //find the last condition
                let mut last_condition = 0;
                for j in (0..len).rev() {
                    if jumps[j] == Jumps::Condition(Condition { addr: 0 }) {
                        last_condition = j;
                        break;
                    }
                }

                if let Jumps::Condition(ref mut condition) = jumps[last_condition] {
                    condition.addr = len;
                }

                //set the address of the forward to last condition
                jumps.push(Jumps::Forward(Forward {
                    back_addr: last_condition,
                }));
            }
            _ => {}
        }
    }

    jumps
}
