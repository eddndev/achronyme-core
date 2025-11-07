use achronyme_eval::evaluator::Evaluator;
use achronyme_types::value::Value;
use std::collections::HashMap;
use std::cell::RefCell;

// ============================================================================
// Global State
// ============================================================================

thread_local! {
    pub static EVALUATOR: RefCell<Evaluator> = RefCell::new(Evaluator::new());
    pub static HANDLES: RefCell<HandleManager> = RefCell::new(HandleManager::new());
}

pub type Handle = u32;

pub struct HandleManager {
    next_handle: Handle,
    values: HashMap<Handle, Value>,
}

impl HandleManager {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            values: HashMap::new(),
        }
    }

    pub fn create(&mut self, value: Value) -> Handle {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.values.insert(handle, value);
        handle
    }

    pub fn get(&self, handle: Handle) -> Option<&Value> {
        self.values.get(&handle)
    }

    pub fn release(&mut self, handle: Handle) {
        self.values.remove(&handle);
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.next_handle = 1;
    }

}
