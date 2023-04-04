use std::os::raw::c_char;
use vim_global::Keycode;
use x11::xlib;

use std::collections::HashSet;

pub struct Display(*mut xlib::Display);

impl Display {
    pub fn new() -> Display {
        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());

            if display.as_ref().is_none() {
                panic!("Couldn't connect");
            }

            Display(display)
        }
    }

    // Taken from mainly device_query
    // https://github.com/ostrosco/device_query/blob/d5ea23be79e96bc70adeb084e486e6da209f36ac/src/device_state/linux/mod.rs
    pub fn query_keymap(&self) -> HashSet<Keycode> {
        let mut keys: HashSet<Keycode> = HashSet::new();
        unsafe {
            let keymap: *mut c_char = [0; 32].as_mut_ptr();

            assert_ne!(xlib::XQueryKeymap(self.0, keymap), 0);

            // Convert to kernel keycode
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

    pub fn move_pointer(&self, x_translation: i32, y_translation: i32) {
        unsafe {
            let root = xlib::XDefaultRootWindow(self.0);

            let mut root_x = 0;
            let mut root_y = 0;
            let mut win_x = 0;
            let mut win_y = 0;
            let mut root_return = 0;
            let mut child_return = 0;
            let mut mask_return = 0;

            assert_ne!(
                xlib::XQueryPointer(
                    self.0,
                    root,
                    &mut root_return,
                    &mut child_return,
                    &mut root_x,
                    &mut root_y,
                    &mut win_x,
                    &mut win_y,
                    &mut mask_return,
                ),
                0
            );

            assert_ne!(
                xlib::XWarpPointer(
                    self.0,
                    root,
                    root,
                    root_x,
                    root_y,
                    0,
                    0,
                    root_x + x_translation,
                    root_y + y_translation,
                ),
                0
            );
        }
    }
}
