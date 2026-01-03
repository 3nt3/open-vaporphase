use slint::{ModelRc, SharedVector, Timer, format};
use std::time::Duration;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    let cycle_duration = Duration::from_secs(15 * 60); // 15 minutes
    let start_time = std::time::Instant::now();
    let total_duration = cycle_duration;

    let mut n_past_temps = 0;
    let mut past_temperatures: [f32; 20] = [0.0; 20];

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

            // Update past temperatures
            if n_past_temps < 20 {
                n_past_temps += 1;
            }
            past_temperatures.rotate_right(1);
            past_temperatures[0] = random_temp;

            main_window.set_past_temperatures(past_temperatures.into());
        },
    );

    main_window.run().unwrap();
}
