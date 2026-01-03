use slint::{Timer, format};
use std::time::Duration;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    let cycle_duration = Duration::from_secs(15 * 60); // 15 minutes
    let start_time = std::time::Instant::now();
    let total_duration = cycle_duration;

    // Update timer
    let main_window_weak = main_window.as_weak();
    let timer = Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100), // Update 10x per second
        move || {
            let main_window = main_window_weak.unwrap();
            let elapsed = start_time.elapsed();

            if elapsed >= total_duration {
                main_window.set_time_remaining("00:00".into());
                main_window.set_is_running(false);

                return;
            }

            let remaining = total_duration - elapsed;
            let remaining_str = format!(
                "{:02}:{:02}",
                remaining.as_secs() / 60,
                remaining.as_secs() % 60
            );

            main_window.set_time_remaining(remaining_str.into());

            // random temp between 170 and 180
            let random_temp: f32 = 170.0 + rand::random::<f32>() * 10.0;
            main_window.set_temperature(format!("{:.1}", random_temp));
        },
    );

    main_window.run().unwrap();
}
