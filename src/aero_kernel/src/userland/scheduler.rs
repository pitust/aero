use alloc::sync::Arc;
use spin::{Mutex, Once};

use hashbrown::HashMap;

use super::process::Process;

static SCHEDULER: Once<Scheduler> = Once::new();

/// Container or a transparent struct containing a hashmap of all of the processes
/// in the scheduler's queue protected by mutex. The hashmap has a key
/// of `usize` (the process id) and a value of a reference-counting pointer
/// to the process or task.
#[repr(transparent)]
struct ProcessContainer(Mutex<HashMap<usize, Arc<Process>>>);

impl ProcessContainer {
    /// Creates a new task container with no tasks by default.
    #[inline]
    fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    /// Registers the provided `process` in the process container.
    #[inline]
    fn register_process(&self, process: Arc<Process>) {
        self.0.lock().insert(process.pid, process);
    }
}

pub struct Scheduler {
    processes: ProcessContainer,
}

impl Scheduler {
    /// Create a new scheduler with no active tasks by default.
    #[inline]
    fn new() -> Self {
        Self {
            processes: ProcessContainer::new(),
        }
    }

    pub fn push(&self, process: Arc<Process>) {
        let context = process.get_context_ref();

        let instruction_ptr = context.get_instruction_ptr();
        let stack_top = context.get_stack_top();
        let rflags = context.rflags;

        self.processes.register_process(process);

        unsafe {
            super::jump_userland(stack_top, instruction_ptr, rflags);
        }
    }
}

/// Get a mutable reference to the active scheduler.
pub fn get_scheduler() -> &'static Scheduler {
    SCHEDULER
        .get()
        .expect("Attempted to get the scheduler before it was initialized")
}

pub fn reschedule() -> bool {
    true
}

/// Initialize the scheduler.
pub fn init() {
    SCHEDULER.call_once(move || Scheduler::new());
}
