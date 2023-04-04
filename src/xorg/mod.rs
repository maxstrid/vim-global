use std::collections::HashSet;
use vim_global::Keycode;
use x11::xlib;

use tracing::{debug, info, trace};

mod error;
use error::XError;

#[derive(Debug, Copy, Clone)]
pub struct PointerInfo {
    pub root_x: i32,
    pub root_y: i32,
    pub win_x: i32,
    pub win_y: i32,
}

pub struct XDisplay<'a>(&'a mut xlib::Display);

impl<'a> XDisplay<'a> {
    #[tracing::instrument]
    pub fn new() -> Result<XDisplay<'a>, XError> {
        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());
            info!("Created _XDisplay");

            if display.as_ref().is_none() {
                return Err(XError::DisplayConnectionError);
            }

            Ok(XDisplay(&mut (*display)))
        }
    }

    pub fn move_pointer(&mut self, x_translation: i32, y_translation: i32) {
        let pointer_info = self.query_pointer();

        self.warp_pointer(
            pointer_info.root_x + x_translation,
            pointer_info.root_y + y_translation,
        );
    }

    pub fn get_default_root_window(&mut self) -> u64 {
        unsafe { xlib::XDefaultRootWindow(self.0) }
    }
    pub fn grab_keyboard(&mut self) {
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

    pub fn ungrab_keyboard(&mut self) {
        unsafe {
            xlib::XUngrabKeyboard(self.0, xlib::CurrentTime);
            xlib::XFlush(self.0);
        }
    }

    pub fn click_mouse(&mut self) -> Result<(), XError> {
        // Not working, unsure why
        let pointer_info = self.query_pointer();

        let mut window = 0;
        let mut revert_to_return = 0;

        unsafe {
            xlib::XGetInputFocus(self.0, &mut window, &mut revert_to_return);
        }

        let mut event = xlib::XEvent {
            button: xlib::XButtonEvent {
                type_: xlib::ButtonPress,
                serial: 0,
                send_event: xlib::True,
                display: self.0,
                window,
                root: self.get_default_root_window(),
                subwindow: window,
                time: xlib::CurrentTime,
                x: pointer_info.win_x,
                y: pointer_info.win_y,
                x_root: pointer_info.root_x,
                y_root: pointer_info.root_y,
                state: xlib::Button1Mask,
                button: xlib::Button1,
                same_screen: xlib::True,
            },
        };

        unsafe {
            let status = xlib::XSendEvent(
                self.0,
                self.get_default_root_window(),
                xlib::False,
                xlib::ButtonPressMask,
                &mut event,
            );

            if status == 0 {
                return Err(XError::WireProtocolFailed);
            }
        }

        Ok(())
    }

    pub fn warp_pointer(&mut self, x_position: i32, y_position: i32) {
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

    pub fn query_keymap(&mut self) -> HashSet<Keycode> {
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

    pub fn query_pointer(&mut self) -> PointerInfo {
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
