extern crate alloc;
use alloc::{boxed::Box, rc::Rc};
use slint::platform::{Platform, software_renderer::MinimalSoftwareWindow};

slint::include_modules!();

pub struct MyPlatform {
    pub window: Rc<MinimalSoftwareWindow>,
    // optional: some timer device from your device's HAL crate
    // timer: embassy_time::Timer, // ... maybe more devices
}

impl Platform for MyPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        // Since on MCUs, there can be only one window, just return a clone of self.window.
        // We'll also use the same window in the event loop.
        Ok(self.window.clone())
    }
    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
}

use slint::platform::software_renderer::Rgb565Pixel;

// In this example, we have two buffer: one is currently displayed, and we are
// rendering into the second one. Hence we use `RepaintBufferType::SwappedBuffers`
let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
    slint::platform::software_renderer::RepaintBufferType::SwappedBuffers
);

const DISPLAY_WIDTH: usize = 480;
const DISPLAY_HEIGHT: usize = 320;
let mut buffer1 = [Rgb565Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT];
let mut buffer2 = [Rgb565Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT];

// ... configure the screen driver to use buffer1 or buffer2 ...

// ... rest of initialization ...

let mut currently_displayed_buffer : &mut [_] = &mut buffer1;
let mut work_buffer : &mut [_] = &mut buffer2;

loop {
    // ...
    // Draw the scene if something needs to be drawn
    window.draw_if_needed(|renderer| {
        // The screen driver might be taking some time to do the swap. We need to wait until
        // work_buffer is ready to be written in
        while is_swap_pending() {}

        // Do the rendering!
        renderer.render_by_line(work_buffer, DISPLAY_WIDTH);

        // tell the screen driver to display the other buffer.
        swap_buffers();

        // Swap the buffer references for our next iteration
        // (this just swap the reference, not the actual data)
        core::mem::swap::<&mut [_]>(&mut work_buffer, &mut currently_displayed_buffer);
    });
    // ...
}

