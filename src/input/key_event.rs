#![allow(unused)]

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Code {
    SingleQuote = 0x27,
    Asterisk = 0x2a,
    Plus = 0x2b,
    Comma = 0x2c,
    Minus = 0x2d,
    Dot = 0x2e,
    Slash = 0x2f,
    N0 = 0x30,
    N1 = 0x31,
    N2 = 0x32,
    N3 = 0x33,
    N4 = 0x34,
    N5 = 0x35,
    N6 = 0x36,
    N7 = 0x37,
    N8 = 0x38,
    N9 = 0x39,
    Semicolon = 0x3b,
    Equal = 0x3d,
    BracketOpen = 0x5b,
    Backslash = 0x5c,
    BracketClose = 0x5d,
    Backtick = 0x60,
    A = 0x61,
    B = 0x62,
    C = 0x63,
    D = 0x64,
    E = 0x65,
    F = 0x66,
    G = 0x67,
    H = 0x68,
    I = 0x69,
    J = 0x6a,
    K = 0x6b,
    L = 0x6c,
    M = 0x6d,
    N = 0x6e,
    O = 0x6f,
    P = 0x70,
    Q = 0x71,
    R = 0x72,
    S = 0x73,
    T = 0x74,
    U = 0x75,
    V = 0x76,
    W = 0x77,
    X = 0x78,
    Y = 0x79,
    Z = 0x7a,
    Escape = 0x80,
    Backspace = 0x81,
    Tab = 0x82,
    Enter = 0x83,
    Control = 0x84,
    Shift = 0x85,
    Alt = 0x87,
    Space = 0x88,
    Capslock = 0x89,
    F1 = 0x8a,
    F2 = 0x8b,
    F3 = 0x8c,
    F4 = 0x8d,
    F5 = 0x8e,
    F6 = 0x8f,
    F7 = 0x90,
    F8 = 0x91,
    F9 = 0x92,
    F10 = 0x93,
    Numberlock = 0x94,
    Scrolllock = 0x95,
    F11 = 0x96,
    F12 = 0x97,
    PreviousTrack = 0x98,
    NextTrack = 0x99,
    Mute = 0x9a,
    Calculator = 0x9b,
    Play = 0x9c,
    Stop = 0x9d,
    VolumeDown = 0x9e,
    VolumeUp = 0x9f,
    WwwHome = 0xa0,
    Home = 0xa1,
    ArrowUp = 0xa2,
    PageUp = 0xa3,
    ArrowLeft = 0xa4,
    ArrowRight = 0xa5,
    End = 0xa6,
    ArrowDown = 0xa7,
    PageDown = 0xa8,
    Insert = 0xa9,
    Delete = 0xaa,
    Gui = 0xab,
    Apps = 0xac,
    Power = 0xad,
    Sleep = 0xae,
    Wake = 0xaf,
    WwwSearch = 0xb0,
    WwwFavorites = 0xb1,
    WwwRefresh = 0xb2,
    WwwStop = 0xb3,
    WwwForward = 0xb4,
    WwwBack = 0xb5,
    MyComputer = 0xb6,
    Email = 0xb7,
    Select = 0xb8,
    PrintScreen = 0xb9,
    Pause = 0xba,
}

#[derive(Clone, Copy)]
pub enum ModVar {
	Left,
	Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PrintVar {
	Regular,
	Keypad,
}

#[derive(Clone, Copy)]
pub enum Key {
	Unknown,
	Modifier(Code, ModVar),
	Printable(Code, PrintVar),
	Control(Code),
	Toggle(Code),
	Media(Code),
	Acpi(Code),
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyState {
	Pressed,
	Released,
}

impl Into<bool> for KeyState {
    fn into(self) -> bool {
        match self {
            Self::Pressed => true,
            Self::Released => false,
        }
    }
}

impl From<bool> for KeyState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Pressed,
            false => Self::Released,
        }
    }
}

#[derive(Clone, Copy)]
pub struct KeyEvent {
	pub state: KeyState,
	pub key: Key,
}