mod xorg;
use std::collections::HashSet;
use x11::xlib;
use vim_global::Keycode;
use xorg::XDisplay;

pub enum InputAction {
    NoAction,
    ClickMouse,
    ScrollMouse,
    FreezeKeyboard,
    UnfreezeKeyboard,
}

pub struct Input {
    display: XDisplay,
    pub mouse_x: i32,
    pub mouse_y: i32,
    action_queue: Vec<InputAction>,
}

impl Input {
    pub fn new() -> Result<Input, Box<dyn std::error::Error>> {
        // TODO: Add native wayland support
        let display = XDisplay::new()?;

        let mouse_x = display.query_pointer().root_x;
        let mouse_y = display.query_pointer().root_y;

        Ok(Input {
            display,
            mouse_x,
            mouse_y,
            action_queue: vec![],
        })
    }

    pub fn update(&mut self) {
        for _ in 0..self.action_queue.len() {
            self.handle_action(self.action_queue.last().unwrap_or(&InputAction::NoAction));
            self.action_queue.remove(0);
        }

        let window_info = self.display.get_window_info();

        if self.mouse_x > window_info.width {
            self.mouse_x = window_info.width;
        } else if self.mouse_x < 0 {
            self.mouse_x = 0;
        } else if self.mouse_y > window_info.height {
            self.mouse_y = window_info.height;
        } else if self.mouse_y < 0 {
            self.mouse_y = 0;
        }
        self.display.warp_pointer(self.mouse_x, self.mouse_y);
    }

    pub fn get_keys(&self) -> HashSet<vim_global::Keycode> {
        self.display.query_keymap()
    }

    pub fn queue_action(&mut self, action: InputAction) {
        self.action_queue.insert(0, action);
    }

    fn handle_action(&self, action: &InputAction) {
        match action {
            InputAction::ClickMouse => {
                self.display.test_fake_button(xlib::Button1, true, 10);
                self.display.test_fake_button(xlib::Button1, false, 10);
            }
            InputAction::FreezeKeyboard => {
                self.display.grab_keyboard();
            }
            InputAction::UnfreezeKeyboard => {
                self.display.ungrab_keyboard();
            }
            _ => (),
        }
    }
}
