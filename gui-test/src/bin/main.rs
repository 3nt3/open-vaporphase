#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use gui_test::display::MyPlatform;
use rtt_target::rprintln;
use slint::platform::{Platform, software_renderer::MinimalSoftwareWindow};

extern crate alloc;
use alloc::{boxed::Box, rc::Rc};

slint::include_modules!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.1.0

    rtt_target::rtt_init_print!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let p = esp_hal::init(config);

    let timg0 = TimerGroup::new(p.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(MyPlatform {
        window: window.clone(),
        // timer: hal::Timer(/*...*/),
        //...
    }))
    .unwrap();

    let ui = MainWindow::new();
    window.set_size(slint::PhysicalSize::new(480, 320));

    // Create a DisplayInterface from SPI and DC pin, with no manual CS control
    let dc = p.pins.gpio1.into_output().unwrap();
    let spi = SpiInterface::new(p.SPI0, dc, &mut buffer);

    // Create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::new(ILI9486Rgb666, spi)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // Clear the display to black
    display.clear(Rgb666::BLACK).unwrap();

    rprintln!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        // Let Slint run the timer hooks and update animations.
        slint::platform::update_timers_and_animations();

        // // Check the touch screen or input device using your driver.
        // if let Some(event) = check_for_touch_event(/*...*/) {
        //     // convert the event from the driver into a `slint::platform::WindowEvent`
        //     // and pass it to the window.
        //     window.try_dispatch_event(event).unwrap();
        // }

        // ... maybe some more application logic ...

        // Draw the scene if something needs to be drawn.
        window.draw_if_needed(|renderer| {
            // see next section about rendering.
            todo!()
        });

        // Try to put the MCU to sleep
        // if !window.has_active_animations() {
        //     if let Some(duration) = slint::platform::duration_until_next_timer_update() {
        //         // ... schedule a timer interrupt in `duration` ...
        //     }
        // }
        Timer::after(Duration::from_millis(16)).await;
    }
}
