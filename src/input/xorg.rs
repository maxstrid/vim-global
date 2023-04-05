use std::collections::HashSet;
use std::fmt;
use vim_global::Keycode;
use x11::xlib;

use tracing::{info, trace};

#[derive(Debug, Copy, Clone)]
pub struct PointerInfo {
    pub root_x: i32,
    pub root_y: i32,
    pub win_x: i32,
    pub win_y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowInfo {
    pub width: i32,
    pub height: i32,
}

pub struct XDisplay(*mut xlib::Display);

impl XDisplay {
    #[tracing::instrument]
    pub fn new() -> Result<XDisplay, XError> {
        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());
            info!("Created _XDisplay");

            if display.as_ref().is_none() {
                return Err(XError::DisplayConnectionError);
            }

            Ok(XDisplay(display))
        }
    }

    pub fn get_default_root_window(&self) -> u64 {
        unsafe { xlib::XDefaultRootWindow(self.0) }
    }
    pub fn grab_keyboard(&self) {
        unsafe {
            xlib::XGrabKeyboard(
                self.0,
                self.get_default_root_window(),
                xlib::True,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                xlib::CurrentTime,
            );
            xlib::XFlush(self.0);
        }
    }

    pub fn ungrab_keyboard(&self) {
        unsafe {
            xlib::XUngrabKeyboard(self.0, xlib::CurrentTime);
            xlib::XFlush(self.0);
        }
    }

    pub fn test_fake_button(&self, button: u32, is_press: bool, delay: u64) {
        let mut press = xlib::True;
        if !is_press {
            press = xlib::False;
        }

        unsafe {
            x11::xtest::XTestFakeButtonEvent(self.0, button, press, delay);
        }
    }

    pub fn get_window_info(&self) -> WindowInfo {
        unsafe {
            let width = xlib::XDisplayWidth(self.0, 0);
            let height = xlib::XDisplayHeight(self.0, 0);

            WindowInfo { width, height }
        }
    }

    pub fn warp_pointer(&self, x_position: i32, y_position: i32) {
        let root = self.get_default_root_window();
        let pointer_info = self.query_pointer();

        unsafe {
            xlib::XWarpPointer(
                self.0,
                root,
                root,
                pointer_info.root_x,
                pointer_info.root_y,
                0,
                0,
                x_position,
                y_position,
            );
        }

        trace!(
            "Warped pointer from ({}, {}) to ({x_position}, {y_position})",
            pointer_info.root_x,
            pointer_info.root_y
        );
    }

    pub fn query_keymap(&self) -> HashSet<Keycode> {
        let mut keys: HashSet<Keycode> = HashSet::new();

        unsafe {
            let keymap: *mut std::os::raw::c_char = [0; 32].as_mut_ptr();

            xlib::XQueryKeymap(self.0, keymap);

            // Taken from mainly device_query
            // https://github.com/ostrosco/device_query/blob/d5ea23be79e96bc70adeb084e486e6da209f36ac/src/device_state/linux/mod.rs
            for (ix, byte) in std::slice::from_raw_parts(keymap, 32).iter().enumerate() {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        //x11 keycode uses kernel keycode with an offset of 8.
                        let x11_key = ix as u8 * 8 + bit;
                        let kernel_key = x11_key - 8;
                        keys.insert(Keycode::from_kernel_code(kernel_key));
                    }
                }
            }
        }

        keys
    }

    pub fn query_pointer(&self) -> PointerInfo {
        let mut root_x = 0;
        let mut root_y = 0;
        let mut win_x = 0;
        let mut win_y = 0;

        let mut root_return = 0;
        let mut child_return = 0;
        let mut mask_return = 0;

        unsafe {
            xlib::XQueryPointer(
                self.0,
                self.get_default_root_window(),
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );
        }

        PointerInfo {
            root_x,
            root_y,
            win_x,
            win_y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum XError {
    DisplayConnectionError,
}

impl fmt::Display for XError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XError::DisplayConnectionError => write!(f, "Couldn't connect to XDisplay server"),
        }
    }
}

impl std::error::Error for XError {}
