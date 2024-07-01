mod calculadora;
mod calcpena;

use chrono::{Utc, Duration, DateTime, Months, Month, TimeZone, offset, Local, TimeDelta};

use eframe::{egui, NativeOptions, run_native};
use crate::calcpena::Calcpena;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 605.0])
            .with_resizable(false),
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    let _ = run_native(
        "Calcpena",
        options,
        Box::new(|cc| Box::new(Calcpena::default()))
    );
}
