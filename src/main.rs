mod xorg;

fn main() {
    println!("Hello, world!");

    let display = xorg::Display::new();
    
    loop {
        let keys = display.query_keymap();

        let mut mode = vim_global::Mode::get_current_mode();

        if keys.contains(&vim_global::Keycode::I) {
            mode = vim_global::Mode::INSERT;
            mode.write();
        }

        if keys.contains(&vim_global::Keycode::Escape) {
            mode = vim_global::Mode::NORMAL;
            mode.write();
        }


        // Timeout
        std::thread::sleep(std::time::Duration::from_millis(1));

        if mode == vim_global::Mode::NORMAL {
            if keys.contains(&vim_global::Keycode::H) {
                display.move_pointer(-1, 0);
            }

            if keys.contains(&vim_global::Keycode::L) {
                display.move_pointer(1, 0);
            }

            if keys.contains(&vim_global::Keycode::J) {
                display.move_pointer(0, -1);
            }

            if keys.contains(&vim_global::Keycode::K) {
                display.move_pointer(0, 1);
            }
        }

        println!("Keys: {keys:?}, Mode: {mode:?}")
    }
}
