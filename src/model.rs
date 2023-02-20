#[allow(dead_code)]

use {
    std::io::Stdout,
    tui::{Terminal, backend::CrosstermBackend}
};


pub enum Keys {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
}

pub enum Modifier {
    One(u8),    // ctrl
    Two(u8),    // shift
    Three(u8),  // alt
    Four(u8),   // meta
    Leader(u8)  // leader
}

pub struct Shortcurt {
    modifier:   Option<Vec<Modifier>>,
    key:        Keys,
    key2:       Option<Vec<Keys>>
}


pub enum LangFunc {
    Rust(fn(terminal: Terminal<CrosstermBackend<Stdout>>) -> ()),
    Lua(fn(terminal: Terminal<CrosstermBackend<Stdout>>) -> ())
}

pub struct Command {
    pub name:   std::string::String,
    pub func:   LangFunc,
    pub sc:     Option<Shortcurt>
}

