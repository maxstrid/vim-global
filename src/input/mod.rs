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
    pub mouse_x_offset: i32,
    pub mouse_y_offset: i32,
    action_queue: Vec<InputAction>,
}

impl Input {
    pub fn new() -> Result<Input, Box<dyn std::error::Error>> {
        // TODO: Add native wayland support
        let display = XDisplay::new()?;

        Ok(Input {
            display,
            mouse_x_offset: 0,
            mouse_y_offset: 0,
            action_queue: vec![],
        })
    }

    pub fn update(&mut self) {
        for _ in 0..self.action_queue.len() {
            self.handle_action(self.action_queue.last().unwrap_or(&InputAction::NoAction));
            self.action_queue.remove(0);
        }

        let mut mouse_x = self.display.query_pointer().root_x + self.mouse_x_offset;
        let mut mouse_y = self.display.query_pointer().root_y + self.mouse_y_offset;

        let window_info = self.display.get_window_info();

        if mouse_x > window_info.width {
            mouse_x = window_info.width;
        } else if mouse_x < 0 {
            mouse_x = 0;
        } else if mouse_y > window_info.height {
            mouse_y = window_info.height;
        } else if mouse_y < 0 {
            mouse_y = 0;
        }
        self.display.warp_pointer(mouse_x, mouse_y);

        self.mouse_x_offset = 0;
        self.mouse_y_offset = 0;
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
