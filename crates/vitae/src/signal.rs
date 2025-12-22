use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::marker::PhantomData;

/// A unique identifier for a signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(usize);

/// A reactive signal that triggers re-renders when updated
pub struct Signal<T> {
    id: SignalId,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for Signal<T> {}

impl<T: Clone + 'static> Signal<T> {
    /// Get the current value of the signal
    pub fn get(&self) -> T {
        SIGNAL_STORAGE.with(|storage| {
            storage
                .borrow()
                .get(&self.id)
                .and_then(|any| any.downcast_ref::<T>())
                .cloned()
                .expect("Signal value not found or type mismatch")
        })
    }

    /// Set a new value for the signal
    pub fn set(&self, value: T) {
        SIGNAL_STORAGE.with(|storage| {
            storage.borrow_mut().insert(self.id, Box::new(value));
        });

        // Trigger redraw
        REQUEST_REDRAW.with(|redraw| redraw.set(true));
    }

    /// Update the signal value using a function
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let current = self.get();
        self.set(f(current));
    }
}

/// Storage for signal values (thread-local)
pub struct SignalStorage {
    values: HashMap<SignalId, Box<dyn Any>>,
}

impl SignalStorage {
    fn new() -> Self {
        SignalStorage {
            values: HashMap::new(),
        }
    }

    fn get(&self, id: &SignalId) -> Option<&Box<dyn Any>> {
        self.values.get(id)
    }

    fn insert(&mut self, id: SignalId, value: Box<dyn Any>) {
        self.values.insert(id, value);
    }

    fn get_or_insert<T: Clone + 'static>(
        &mut self,
        id: SignalId,
        init: impl FnOnce() -> T,
    ) -> &Box<dyn Any> {
        self.values.entry(id).or_insert_with(|| Box::new(init()))
    }

    /// Clear all signal values (used when resetting between frames)
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

// Thread-local storage
thread_local! {
    static SIGNAL_STORAGE: RefCell<SignalStorage> = RefCell::new(SignalStorage::new());
    static SIGNAL_COUNTER: Cell<usize> = Cell::new(0);
    static REQUEST_REDRAW: Cell<bool> = Cell::new(false);
}

/// Create a new signal with an initial value
///
/// This should be called during the view function. The signal counter is reset
/// before each render to ensure consistent signal IDs.
pub fn use_signal<T: Clone + 'static>(init: impl FnOnce() -> T) -> Signal<T> {
    SIGNAL_STORAGE.with(|storage| {
        let id = SIGNAL_COUNTER.with(|c| {
            let id = c.get();
            c.set(id + 1);
            SignalId(id)
        });

        // Initialize if first time (or get existing value)
        storage.borrow_mut().get_or_insert(id, init);

        Signal {
            id,
            _phantom: PhantomData,
        }
    })
}

/// Reset the signal counter (called before each render)
pub(crate) fn reset_signal_counter() {
    SIGNAL_COUNTER.with(|c| c.set(0));
}

/// Check if a redraw was requested by a signal update
pub(crate) fn take_redraw_request() -> bool {
    REQUEST_REDRAW.with(|redraw| {
        let value = redraw.get();
        redraw.set(false);
        value
    })
}

/// Clear all signal storage (useful for cleanup)
pub(crate) fn clear_signals() {
    SIGNAL_STORAGE.with(|storage| storage.borrow_mut().clear());
}
