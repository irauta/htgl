
use core::cell::RefCell;
use std::rc::Rc;

pub struct SharedContextState {
    pub context_alive: bool
}

impl SharedContextState {
    pub fn new() -> SharedContextState {
        SharedContextState {
            context_alive: true
        }
    }
}

pub struct RegistrationHandle {
    context_shared: Rc<RefCell<SharedContextState>>
}

impl RegistrationHandle {
    pub fn new(context_shared: Rc<RefCell<SharedContextState>>) -> RegistrationHandle {
        RegistrationHandle { context_shared: context_shared }
    }

    pub fn context_alive(&self) -> bool {
        self.context_shared.borrow().context_alive
    }
}