use std::{rc::Rc, cell::RefCell};

use iced_x86::code_asm::CodeAssembler;

use crate::{NDSState, Addr};

use self::block::CodeBlock;

pub mod cacher;
pub mod block;

pub fn compile(bytes: &[u8]) -> CodeBlock {
    todo!();
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
}
