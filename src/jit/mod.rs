use std::{rc::Rc, cell::RefCell};

use iced_x86::code_asm::CodeAssembler;

use crate::{NDSState, Addr};

use self::block::CodeBlock;

pub mod cacher;
pub mod block;

pub fn compile(state: Rc<RefCell<NDSState>>) -> CodeBlock {
    let mut jit = JIT::new(state);

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
    pub x86: CodeAssembler,
}

impl JIT {
    const BITNESS: u32 = 64;

    fn new(state: Rc<RefCell<NDSState>>) -> Self {

        todo!()
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
    }
}
