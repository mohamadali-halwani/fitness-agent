use eframe::egui;
use std::sync::mpsc::{self, Receiver};
use std::thread;

struct PlannerApp {
    name: String,
    age: String,
    weight: String,
    height: String,
    goal: String,
    plan: String,
    generating: bool,
    rx: Option<Receiver<String>>,
}

impl PlannerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180);
        cc.egui_ctx.set_style(style);

        Self {
            name: String::new(),
            age: String::new(),
            weight: String::new(),
            height: String::new(),
            goal: String::new(),
            plan: String::new(),
            generating: false,
            rx: None,
        }
    }
}

impl eframe::App for PlannerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Daily Fitness Planner");
            ui.label("Enter your details:");
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Age:");
                ui.text_edit_singleline(&mut self.age);
            });
            ui.horizontal(|ui| {
                ui.label("Weight (kg):");
                ui.text_edit_singleline(&mut self.weight);
            });
            ui.horizontal(|ui| {
                ui.label("Height (cm):");
                ui.text_edit_singleline(&mut self.height);
            });
            ui.horizontal(|ui| {
                ui.label("Goal:");
                ui.text_edit_singleline(&mut self.goal);
            });
            if ui.button("Generate Plan").clicked() && !self.generating {
                let prompt = format!(
                    "Provide today's exercise schedule and diet plan for {}. Age: {}. Weight: {}kg. Height: {}cm. Goal: {}.",
                    self.name, self.age, self.weight, self.height, self.goal
                );
                let (tx, rx) = mpsc::channel();
                self.generating = true;
                self.rx = Some(rx);
                thread::spawn(move || {
                    let plan = match generate_plan(&prompt) {
                        Ok(p) => p,
                        Err(e) => format!("Failed to generate plan: {e}")
                    };
                    let _ = tx.send(plan);
                });
            }
            if let Some(rx) = &self.rx {
                if let Ok(plan) = rx.try_recv() {
                    self.plan = plan;
                    self.generating = false;
                    self.rx = None;
                }
            }
            if self.generating {
                ui.label("Generating plan...");
            }
            if !self.plan.is_empty() {
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(&self.plan);
                });
            }
        });
    }
}

fn generate_plan(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = reqwest::blocking::Client::new();
    let res: serde_json::Value = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 500,
        }))
        .send()?
        .json()?;
    let content = res["choices"][0]["message"]["content"].as_str().unwrap_or("No response");
    Ok(content.to_string())
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Exercise Planner",
        options,
        Box::new(|cc| Box::new(PlannerApp::new(cc))),
    )
    .expect("failed to start eframe");
}
