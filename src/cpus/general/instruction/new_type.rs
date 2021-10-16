use core::convert::From;

use crate::{
    ram::data_types::DataType,
    cpus::general::condition_code_flag::ConditionCodeFlag,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Instruction(u32);

impl Instruction {
    pub fn get_value_as_u32(&self) -> u32 {
        self.0
    }

    pub fn get_condition_code_flag(&self) -> ConditionCodeFlag {
        match (self.0.clone() >> 28) & 0b1111 {
            0b0000 => ConditionCodeFlag::EQ,
            0b0001 => ConditionCodeFlag::NE,
            0b0010 => ConditionCodeFlag::CS,
            0b0011 => ConditionCodeFlag::CC,
            0b0100 => ConditionCodeFlag::MI,
            0b0101 => ConditionCodeFlag::PL,
            0b0110 => ConditionCodeFlag::VS,
            0b0111 => ConditionCodeFlag::VC,
            0b1001 => ConditionCodeFlag::HI,
            0b1010 => ConditionCodeFlag::LS,
            0b1011 => ConditionCodeFlag::GE,
            0b1100 => ConditionCodeFlag::LT,
            0b1101 => ConditionCodeFlag::LE,
            0b1110 => ConditionCodeFlag::AL,
            _other => unreachable!("[INSTRUCTION ERROR]: Instruction has unknown condition flag!"),
        }
    }
}

impl From<u32> for Instruction {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

impl From<DataType> for Instruction {
    fn from(data_type: DataType) -> Self {
        Self(data_type.get_value_as_u32())
    }
}

#[cfg(test)]
mod tests {

    use super::Instruction;
    use crate::cpus::general::condition_code_flag::ConditionCodeFlag;

    #[test]
    fn get_value_as_u32() {
        let instruction = Instruction::from(42);

        assert_eq!(instruction.get_value_as_u32(), 42);
    }

    #[test]
    fn get_condition_code_flag() {
        let instruction1  = Instruction::from(0b0000_0000_0000_0000_0000_0000_0000_0000);
        let instruction2  = Instruction::from(0b0001_0000_0000_0000_0000_0000_0000_0000);
        let instruction3  = Instruction::from(0b0010_0000_0000_0000_0000_0000_0000_0000);
        let instruction4  = Instruction::from(0b0011_0000_0000_0000_0000_0000_0000_0000);
        let instruction5  = Instruction::from(0b0100_0000_0000_0000_0000_0000_0000_0000);
        let instruction6  = Instruction::from(0b0101_0000_0000_0000_0000_0000_0000_0000);
        let instruction7  = Instruction::from(0b0110_0000_0000_0000_0000_0000_0000_0000);
        let instruction8  = Instruction::from(0b0111_0000_0000_0000_0000_0000_0000_0000);
        let instruction9  = Instruction::from(0b1001_0000_0000_0000_0000_0000_0000_0000);
        let instruction10 = Instruction::from(0b1010_0000_0000_0000_0000_0000_0000_0000);
        let instruction11 = Instruction::from(0b1011_0000_0000_0000_0000_0000_0000_0000);
        let instruction12 = Instruction::from(0b1100_0000_0000_0000_0000_0000_0000_0000);
        let instruction13 = Instruction::from(0b1101_0000_0000_0000_0000_0000_0000_0000);
        let instruction14 = Instruction::from(0b1110_0000_0000_0000_0000_0000_0000_0000);

        assert_eq!(instruction1.get_condition_code_flag(), ConditionCodeFlag::EQ);
        assert_eq!(instruction2.get_condition_code_flag(), ConditionCodeFlag::NE);
        assert_eq!(instruction3.get_condition_code_flag(), ConditionCodeFlag::CS);
        assert_eq!(instruction4.get_condition_code_flag(), ConditionCodeFlag::CC);
        assert_eq!(instruction5.get_condition_code_flag(), ConditionCodeFlag::MI);
        assert_eq!(instruction6.get_condition_code_flag(), ConditionCodeFlag::PL);
        assert_eq!(instruction7.get_condition_code_flag(), ConditionCodeFlag::VS);
        assert_eq!(instruction8.get_condition_code_flag(), ConditionCodeFlag::VC);
        assert_eq!(instruction9.get_condition_code_flag(), ConditionCodeFlag::HI);
        assert_eq!(instruction10.get_condition_code_flag(), ConditionCodeFlag::LS);
        assert_eq!(instruction11.get_condition_code_flag(), ConditionCodeFlag::GE);
        assert_eq!(instruction12.get_condition_code_flag(), ConditionCodeFlag::LT);
        assert_eq!(instruction13.get_condition_code_flag(), ConditionCodeFlag::LE);
        assert_eq!(instruction14.get_condition_code_flag(), ConditionCodeFlag::AL);
    }
}
