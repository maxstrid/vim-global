use tracing::info;
use tracing_subscriber;

mod input;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut input = input::Input::new().unwrap();
    let mut coefficient = 1;
    let mut keys = input.get_keys();

    loop {
        let last_keys = keys.clone();
        keys = input.get_keys();

        if keys.is_empty() {
            continue;
        }

        let mut mode = vim_global::Mode::get_current_mode();

        if last_keys != keys {
            if keys.contains(&vim_global::Keycode::Control)
                && keys.contains(&vim_global::Keycode::Super)
                && keys.contains(&vim_global::Keycode::Q)
            {
                info!("Ctrl + Super + Q found, quitting.");
                break;
            }

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
        }

        match mode {
            vim_global::Mode::NORMAL => {
                if keys.contains(&vim_global::Keycode::Key1) {
                    coefficient = 1;
                }

                if keys.contains(&vim_global::Keycode::Key2) {
                    coefficient = 2;
                }

                if keys.contains(&vim_global::Keycode::Key3) {
                    coefficient = 3;
                }

                if keys.contains(&vim_global::Keycode::H) {
                    input.mouse_x_offset += -1 * coefficient;
                }

                if keys.contains(&vim_global::Keycode::L) {
                    input.mouse_x_offset += 1 * coefficient;
                }

                if keys.contains(&vim_global::Keycode::J) {
                    input.mouse_y_offset += 1 * coefficient;
                }

                if keys.contains(&vim_global::Keycode::K) {
                    input.mouse_y_offset += -1 * coefficient;
                }

                if keys.contains(&vim_global::Keycode::Space) {
                    input.queue_action(input::InputAction::ClickMouse);
                }

                input.queue_action(input::InputAction::FreezeKeyboard);
            }
            vim_global::Mode::INSERT => {
                input.queue_action(input::InputAction::UnfreezeKeyboard);
            }
        }

        // Timeout
        std::thread::sleep(std::time::Duration::from_millis(1));

        input.update();
    }
}
