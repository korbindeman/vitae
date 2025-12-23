use std::any::Any;
use std::rc::Rc;

/// Result of handling an event, controls propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Continue propagating the event.
    Continue,
    /// Stop propagating the event.
    Stop,
}

/// A keyboard key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// A character key (letters, numbers, symbols).
    Character(String),
    /// Named/special keys.
    Named(NamedKey),
    /// Unknown or unrecognized key.
    Unknown,
}

/// Named (non-character) keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedKey {
    Enter,
    Tab,
    Space,
    Backspace,
    Delete,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Shift,
    Control,
    Alt,
    Meta,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// All possible events.
#[derive(Debug, Clone)]
pub enum Event {
    Click { button: MouseButton },
    MouseDown { button: MouseButton },
    MouseUp { button: MouseButton },
    KeyDown { key: Key, repeat: bool },
    KeyUp { key: Key },
}

/// Event handler that can update the model.
pub type EventHandler = Rc<dyn Fn(&mut dyn Any, &Event) -> EventResult>;
