#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL
}

//trait that tells our program how to convert from a byte to a specific opcode
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        return match v {
            0 => Opcode::HLT,
            _ => Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}