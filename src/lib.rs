use std::fs::File;
use std::io::Read;
use tracing::{info, trace};

use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum Keycode {
    Key0 = 11,
    Key1 = 2,
    Key2 = 3,
    Key3 = 4,
    Key4 = 5,
    Key5 = 6,
    Key6 = 7,
    Key7 = 8,
    Key8 = 9,
    Key9 = 10,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    Escape = 1,
    Minus = 12,
    Equal = 13,
    Backspace = 14,
    Tab = 15,
    LeftBrace = 26,
    RightBrace = 27,
    Enter = 28,
    Control = 29,
    Semicolon = 39,
    Apostrophe = 40,
    Backtick = 41,
    Shift = 42,
    Backslash = 43,
    Comma = 51,
    Dot = 52,
    Slash = 53,
    Asterisk = 55,
    Alt = 56,
    Space = 57,
    Capslock = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    F11 = 69,
    F12 = 70,
    KeyUp = 103,
    KeyLeft = 105,
    KeyRight = 106,
    KeyDown = 108,
    Super = 125,
    Unknown = 999,
}

impl Keycode {
    pub fn to_x11(&self) -> u32 {
        *self as u32
    }
}

const MODE_FILE_PATH: &str = "/tmp/vim_global_mode_current";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    INSERT,
    NORMAL,
}

impl Mode {
    pub fn get_current_mode() -> Mode {
        let mode_file = File::open(MODE_FILE_PATH);

        if let Ok(mut file) = mode_file {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            trace!("Read Mode {content} from {MODE_FILE_PATH}");

            match content.as_str() {
                "INSERT" => return Mode::INSERT,
                _ => return Mode::NORMAL,
            }
        } else if let Err(err) = mode_file {
            if err.kind() == std::io::ErrorKind::NotFound {
                File::create(MODE_FILE_PATH).unwrap();
                info!("Created file {MODE_FILE_PATH}");
            }
        }

        Mode::NORMAL
    }

    pub fn write(&self) {
        let mut content = "NORMAL";

        match self {
            Mode::INSERT => content = "INSERT",
            Mode::NORMAL => (),
        };

        info!("Wrote mode {content} to {MODE_FILE_PATH}");

        std::fs::write(MODE_FILE_PATH, content).unwrap();
    }
}
