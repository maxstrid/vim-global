use tracing::info;
use tracing_subscriber;

mod input;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut input = input::Input::new().unwrap();

    loop {
        let keys = input.get_keys();

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

        match mode {
            vim_global::Mode::NORMAL => {
                if keys.contains(&vim_global::Keycode::H) {
                    input.mouse_x += -1;
                }

                if keys.contains(&vim_global::Keycode::L) {
                    input.mouse_x += 1;
                }

                if keys.contains(&vim_global::Keycode::J) {
                    input.mouse_y += -1;
                }

                if keys.contains(&vim_global::Keycode::K) {
                    input.mouse_y += 1;
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
