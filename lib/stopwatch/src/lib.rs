#![feature(const_fn)]

use fiber::FiberId;
use once_cell::sync::OnceCell;
use serde_derive::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::mem;
use parking_lot::Mutex;
use std::time::Duration;

#[cfg(target_os="windows")]
#[path="windows.rs"]
pub mod platform;
pub mod stats;

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(Context::new());
}

unsafe fn init_context_map() -> &'static Mutex<HashMap<FiberId, Context>> {
    static mut CONTEXT_MAP: OnceCell<Mutex<HashMap<FiberId, Context>>> = OnceCell::new();
    CONTEXT_MAP.get_or_init(|| {
        Mutex::new(HashMap::with_capacity(1024))
    })
}
unsafe fn init_events() -> &'static Mutex<Vec<Event>> {
    static mut EVENTS: OnceCell<Mutex<Vec<Event>>> = OnceCell::new();
    EVENTS.get_or_init(|| {
        Mutex::new(Vec::new())
    })
}

/// Swaps the currently tracked execution context with the specified context.
pub fn switch_context(old: FiberId, new: FiberId) {
    unsafe { init_context_map() };
    with_context(|stack| {
        let timestamp = platform::timestamp();

        // Push an end event for each of the time slices.
        for stopwatch in stack.iter().rev() {
            push_event(Event {
                name: stopwatch.name,
                cat: String::new(),
                ph: "E",
                ts: timestamp,
                tid: platform::thread_id(),
                pid: 0,
            });
        }
    });

    let mut context_map = unsafe {
        init_context_map().lock()
    };

    let new_context = context_map.remove(&new).unwrap_or(Context::new());
    let old_context = with_context(move |context| {
        let mut new_context = new_context;
        mem::swap(context, &mut new_context);
        new_context
    });

    context_map.insert(old, old_context);

    with_context(|stack| {
        let timestamp = platform::timestamp();

        // Push an end event for each of the time slices.
        for stopwatch in stack.iter() {
            push_event(Event {
                name: stopwatch.name,
                cat: String::new(),
                ph: "B",
                ts: timestamp,
                tid: platform::thread_id(),
                pid: 0,
            });
        }
    });
}

/// Writes the events history to a string.
pub fn write_events_to_string() -> String {
    let events = unsafe {
        init_events().lock()
    };
    serde_json::to_string(&*events).unwrap()
}

pub struct Stopwatch {
    name: &'static str,
}

impl Stopwatch {
    pub fn new(name: &'static str) -> Stopwatch {
        push_event(Event {
            name: name,
            cat: String::new(),
            ph: "B",
            ts: platform::timestamp(),
            tid: platform::thread_id(),
            pid: 0, // TODO: Do we care about tracking process ID?
        });

        with_context(|stack| {
            stack.push(StopwatchData { name: name });
        });

        Stopwatch {
            name: name,
        }
    }

    pub fn with_budget(name: &'static str, _budget: Duration) -> Stopwatch {
        // TODO: We should actually do something with the budget, right?
        Stopwatch::new(name)
    }
}

impl Drop for Stopwatch {
    fn drop(&mut self) {
        with_context(|stack| {
            let stopwatch = stack.pop().expect("No stopwatch popped, stack is corrupted");
            assert_eq!(self.name, stopwatch.name, "Stack got corrupted I guess");
        });

        push_event(Event {
            name: self.name,
            cat: String::new(),
            ph: "E",
            ts: platform::timestamp(),
            tid: platform::thread_id(),
            pid: 0, // TODO: Do we care about tracking process ID?
        });
    }
}

#[derive(Debug, Serialize)]
struct Event {
    /// Human-readable name for the event.
    name: &'static str,

    /// Event category.
    cat: String,

    /// Event phase (i.e. the event type).
    ph: &'static str,

    /// Timestamp in microseconds.
    ts: i64,

    /// Process ID for the event.
    pid: usize,

    /// Thread ID for the event.
    tid: usize,
}

fn push_event(event: Event) {
    unsafe { init_events() };
    let mut events = unsafe {
        init_events().lock()
    };
    events.push(event);
}

#[derive(Debug, Clone, Copy)]
struct StopwatchData {
    name: &'static str,
}

type Context = Vec<StopwatchData>;

fn with_context<F, T>(func: F) -> T
    where F: FnOnce(&mut Context) -> T
{
    CONTEXT.with(move |context_cell| {
        let mut context = context_cell.borrow_mut();
        func(&mut *context)
    })
}

pub struct PrettyDuration(pub Duration);

impl Display for PrettyDuration {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let mins = self.0.as_secs() / 60;
        let secs = self.0.as_secs() % 60;
        let millis = self.0.subsec_nanos() as u64 / 1_000_000;
        let micros = (self.0.subsec_nanos() / 1_000) % 1_000;

        if mins > 0 {
            write!(formatter, "{}m {}s {}.{}ms", mins, secs, millis, micros)
        } else if secs > 0 {
            write!(formatter, "{}s {}.{}ms", secs, millis, micros)
        } else {
            write!(formatter, "{}.{}ms", millis, micros)
        }
    }
}
