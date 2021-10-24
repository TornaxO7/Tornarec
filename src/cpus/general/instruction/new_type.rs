use crate::{
    cpus::general::condition_code_flag::ConditionCodeFlag,
    ram::Address,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Instruction {
    pub address: Address,
    pub val: u32,
}

impl Instruction {
    pub fn get_condition_code_flag(&self) -> ConditionCodeFlag {
        match (self.val >> 28) & 0b1111 {
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

#[cfg(test)]
mod tests {

    use crate::{
        cpus::general::{
            condition_code_flag::ConditionCodeFlag,
            Instruction,
        },
        ram::Address,
    };

    #[test]
    fn get_condition_code_flag() {
        let instruction1 = Instruction {
            address: Address::from(0),
            val: 0b0000_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction2 = Instruction {
            address: Address::from(0),
            val: 0b0001_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction3 = Instruction {
            address: Address::from(0),
            val: 0b0010_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction4 = Instruction {
            address: Address::from(0),
            val: 0b0011_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction5 = Instruction {
            address: Address::from(0),
            val: 0b0100_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction6 = Instruction {
            address: Address::from(0),
            val: 0b0101_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction7 = Instruction {
            address: Address::from(0),
            val: 0b0110_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction8 = Instruction {
            address: Address::from(0),
            val: 0b0111_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction9 = Instruction {
            address: Address::from(0),
            val: 0b1001_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction10 = Instruction {
            address: Address::from(0),
            val: 0b1010_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction11 = Instruction {
            address: Address::from(0),
            val: 0b1011_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction12 = Instruction {
            address: Address::from(0),
            val: 0b1100_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction13 = Instruction {
            address: Address::from(0),
            val: 0b1101_0000_0000_0000_0000_0000_0000_0000,
        };
        let instruction14 = Instruction {
            address: Address::from(0),
            val: 0b1110_0000_0000_0000_0000_0000_0000_0000,
        };

        assert_eq!(
            instruction1.get_condition_code_flag(),
            ConditionCodeFlag::EQ
        );
        assert_eq!(
            instruction2.get_condition_code_flag(),
            ConditionCodeFlag::NE
        );
        assert_eq!(
            instruction3.get_condition_code_flag(),
            ConditionCodeFlag::CS
        );
        assert_eq!(
            instruction4.get_condition_code_flag(),
            ConditionCodeFlag::CC
        );
        assert_eq!(
            instruction5.get_condition_code_flag(),
            ConditionCodeFlag::MI
        );
        assert_eq!(
            instruction6.get_condition_code_flag(),
            ConditionCodeFlag::PL
        );
        assert_eq!(
            instruction7.get_condition_code_flag(),
            ConditionCodeFlag::VS
        );
        assert_eq!(
            instruction8.get_condition_code_flag(),
            ConditionCodeFlag::VC
        );
        assert_eq!(
            instruction9.get_condition_code_flag(),
            ConditionCodeFlag::HI
        );
        assert_eq!(
            instruction10.get_condition_code_flag(),
            ConditionCodeFlag::LS
        );
        assert_eq!(
            instruction11.get_condition_code_flag(),
            ConditionCodeFlag::GE
        );
        assert_eq!(
            instruction12.get_condition_code_flag(),
            ConditionCodeFlag::LT
        );
        assert_eq!(
            instruction13.get_condition_code_flag(),
            ConditionCodeFlag::LE
        );
        assert_eq!(
            instruction14.get_condition_code_flag(),
            ConditionCodeFlag::AL
        );
    }
}
