use std::{rc::Rc, cell::RefCell};

use iced_x86::code_asm::CodeAssembler;

use crate::{NDSState, nds_components::cpus::{architecture::Architecture, OperatingState}, Addr};

use self::block::CodeBlock;

pub mod cacher;
pub mod block;
mod arm;
mod thumb;

pub fn compile(state: Rc<RefCell<NDSState>>, cpu: Rc<RefCell<Architecture>>) -> CodeBlock {
    let mut jit = JIT::new(state, cpu);

    jit.compile()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmitStatus {
    ReachedBranch,
    Null,
}

pub struct JIT {
    start_pc: Addr,
    state: Rc<RefCell<NDSState>>,
    cpu: Rc<RefCell<Architecture>>,
    pub x86: CodeAssembler,
}

impl JIT {
    const BITNESS: u32 = 64;

    fn new(state: Rc<RefCell<NDSState>>, cpu: Rc<RefCell<Architecture>>) -> Self {
        let start_pc = cpu.borrow().pc as Addr;

        Self {
            start_pc,
            state,
            cpu,
            x86: CodeAssembler::new(Self::BITNESS).unwrap(),
        }
    }

    fn compile(&mut self) -> CodeBlock {
        self.prolog();
        self.recompile_block();
        self.epilog();

        todo!()
    }

    fn prolog(&mut self) {
        use iced_x86::code_asm::*;
        self.x86.push(rbp).unwrap();
        self.x86.mov(rbp, rsp).unwrap();

    }

    fn epilog(&mut self) {
        use iced_x86::code_asm::*;
        self.x86.mov(rsp, rbp).unwrap();
        self.x86.pop(rbp).unwrap();
        self.x86.ret().unwrap();

    }

    fn recompile_block(&mut self) {
        let op_state = self.cpu.borrow().op_state.clone();
        let pc = self.cpu.borrow().pc as Addr;

        match op_state {
            OperatingState::Arm => self.compile_arm_block(pc),
            OperatingState::Thumb => self.compile_thumb_block(pc),
        }
    }
}
