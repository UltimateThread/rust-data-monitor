use eframe::egui;

pub struct ApplicationState {
    pub ctx: Option<egui::Context>,
    pub field: i128,
    pub data: Option<Vec<[f64; 2]>>,
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            ctx: None,
            field: 0,
            data: Some(vec![]),
        }
    }
}