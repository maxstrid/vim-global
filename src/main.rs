use tracing::info;
use tracing_subscriber;

mod xorg;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut display = xorg::XDisplay::new().unwrap();

    loop {
        let keys = display.query_keymap();

        if keys.contains(&vim_global::Keycode::Control)
            && keys.contains(&vim_global::Keycode::Super)
            && keys.contains(&vim_global::Keycode::Q)
        {
            info!("Ctrl + Super + Q found, quitting.");
            break;
        }

        let mut mode = vim_global::Mode::get_current_mode();

        if keys.contains(&vim_global::Keycode::I) {
            mode = vim_global::Mode::INSERT;
            mode.write();
        }

        if keys.contains(&vim_global::Keycode::Escape)
            && keys.contains(&vim_global::Keycode::Control)
        {
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

            if keys.contains(&vim_global::Keycode::Space) {
                display.click_mouse();
            }

            display.grab_keyboard();
        } else {
            display.ungrab_keyboard();
        }
    }
}
