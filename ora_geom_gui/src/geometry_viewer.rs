use eframe::App;

pub struct GeometryViewer {
    pub name: String,
    pub age: u32,
}

impl GeometryViewer {
    pub fn new() -> Self {
        Self {
            name: String::from("Arthur"),
            age: 42,
        }
    }
}

impl App for GeometryViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!(
                "Hello '{name}', age {age}",
                name = self.name,
                age = self.age
            ));
        });
    }
}
