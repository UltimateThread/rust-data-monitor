mod application_state;
mod data_monitor;

use crate::data_monitor::DataMonitor;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Data Monitor",
        options,
        Box::new(move |cc| Box::new(DataMonitor::new(cc))),
    )
}
