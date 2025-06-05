use eframe::egui;
use std::time::Instant;

struct App {
    last_press: Instant,
    bpm: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            last_press: Instant::now(),
            bpm: 0.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add_sized([200.0, 80.0], egui::Button::new("Tap")).clicked() {
                let now = Instant::now();
                let delta = now.duration_since(self.last_press);
                self.last_press = now;

                if delta.as_secs_f32() < 2.0 {
                    self.bpm = 60.0 / delta.as_secs_f32();
                }
            }

            ui.label(format!("BPM: {:.2}", self.bpm));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([220.0, 120.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "BPM",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
