use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};
use egui_plot::{Line, Plot, PlotPoints};
use crate::application_state::ApplicationState;

pub struct DataMonitor {
    application_state: Arc<Mutex<ApplicationState>>,
}

impl DataMonitor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = Arc::new(Mutex::new(ApplicationState::new()));
        state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());

        let state_clone = state.clone();
        thread::spawn(move || {
            loop {
                let mut csv_data: Vec<[f64; 2]> = load_csv_data();
                let mut local_data: Vec<[f64; 2]> = vec![];

                for data in csv_data {
                    thread::sleep(Duration::from_millis(100));
                    local_data.push(data);
                    if state_clone.lock().unwrap().data.is_some() {
                        state_clone.lock().unwrap().data = Some(local_data.clone());
                        state_clone.lock().unwrap().field = local_data.len() as i128;
                    }
                    // state_clone.lock().unwrap().field = local_data.len() as i128;
                    // state_clone.lock().unwrap().data = Some(local_data.clone());

                    let ctx = &state_clone.lock().unwrap().ctx;
                    match ctx {
                        Some(x) => x.request_repaint(),
                        None => panic!("error in Option<>"),
                    }
                }

                thread::sleep(Duration::from_secs(100));
            }
        });

        Self {
            application_state: state,
        }
    }
}

impl eframe::App for DataMonitor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let local_state = self.application_state.lock().unwrap();

            ui.label(format!("{}", local_state.field));

            draw_graph(ui, local_state);
        });
    }
}

fn draw_graph(ui: &mut Ui, local_state: MutexGuard<ApplicationState>) {
    let plot = Plot::new("Data");
    plot.show(ui, |plot_ui| {
        // Points::
        // plot_ui.points(Points::new(PlotPoints::from(graph)).radius(10.));
        match local_state.data.as_ref() {
            Some(data) => {
                plot_ui.line(Line::new(PlotPoints::from(data.clone())).name("curve"));
            }
            None => {}
        }
    });
}

fn load_csv_data() -> Vec<[f64; 2]> {
    let mut local_data: Vec<[f64; 2]> = vec![];

    let reader = csv::Reader::from_path("test.csv");
    match reader {
        Ok(mut reader) => {
            for result in reader.records() {
                match result {
                    Ok(_) => {
                        let record = result;
                        match record {
                            Ok(record) => {
                                let str_time = record.get(0);
                                let str_rpm = record.get(1);

                                if str_time.is_some() && str_rpm.is_some() {
                                    let time: f64 = str_time.unwrap().parse().unwrap();
                                    let value: f64 = str_rpm.unwrap().parse().unwrap();

                                    local_data.push([time, value]);
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => { println!("Failed to Open CSV File") }
    }

    local_data
}
