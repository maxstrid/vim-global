use std::fs::File;
use std::io::Read;
use tracing::{info, trace};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keycode {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
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
    Z,
    Escape,
    Minus,
    Equal,
    Backspace,
    Tab,
    LeftBrace,
    RightBrace,
    Enter,
    Control,
    Semicolon,
    Apostrophe,
    Backtick,
    Shift,
    Backslash,
    Comma,
    Dot,
    Slash,
    Asterisk,
    Alt,
    Space,
    Capslock,
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
    Super,
    Unknown,
}

impl Keycode {
    pub fn from_kernel_code(kernel_code: u8) -> Keycode {
        match kernel_code {
            125 => Keycode::Super,
            1 => Keycode::Escape,
            2 => Keycode::Key1,
            3 => Keycode::Key2,
            4 => Keycode::Key3,
            5 => Keycode::Key4,
            6 => Keycode::Key5,
            7 => Keycode::Key6,
            8 => Keycode::Key7,
            9 => Keycode::Key8,
            10 => Keycode::Key9,
            11 => Keycode::Key0,
            12 => Keycode::Minus,
            13 => Keycode::Equal,
            14 => Keycode::Backspace,
            15 => Keycode::Tab,
            16 => Keycode::Q,
            17 => Keycode::W,
            18 => Keycode::E,
            19 => Keycode::R,
            20 => Keycode::T,
            21 => Keycode::Y,
            22 => Keycode::U,
            23 => Keycode::I,
            24 => Keycode::O,
            25 => Keycode::P,
            30 => Keycode::A,
            31 => Keycode::S,
            32 => Keycode::D,
            33 => Keycode::F,
            34 => Keycode::G,
            35 => Keycode::H,
            36 => Keycode::J,
            37 => Keycode::K,
            38 => Keycode::L,
            44 => Keycode::Z,
            45 => Keycode::X,
            46 => Keycode::C,
            47 => Keycode::V,
            48 => Keycode::B,
            49 => Keycode::N,
            50 => Keycode::M,
            26 => Keycode::LeftBrace,
            27 => Keycode::RightBrace,
            28 => Keycode::Enter,
            29 | 97 => Keycode::Control,
            39 => Keycode::Semicolon,
            40 => Keycode::Apostrophe,
            41 => Keycode::Backtick,
            42 | 54 => Keycode::Shift,
            43 => Keycode::Backslash,
            51 => Keycode::Comma,
            52 => Keycode::Dot,
            53 => Keycode::Slash,
            55 => Keycode::Asterisk,
            56 => Keycode::Alt,
            57 => Keycode::Space,
            58 => Keycode::Capslock,
            59 => Keycode::F1,
            60 => Keycode::F2,
            61 => Keycode::F3,
            62 => Keycode::F4,
            63 => Keycode::F5,
            64 => Keycode::F6,
            65 => Keycode::F7,
            66 => Keycode::F8,
            67 => Keycode::F9,
            68 => Keycode::F10,
            69 => Keycode::F11,
            70 => Keycode::F12,
            _ => Keycode::Unknown,
        }
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
