use egui::{ahash::HashMap, Button, Color32, Context, Layout, Window};

use crate::{geometry_viewer::InputQuery, sdo_geometry::SdoGeometry};

pub struct QueryWindow<'a> {
    pub queries: &'a mut HashMap<String, Query>,
    pub input_query: &'a mut InputQuery,
}

impl<'a> QueryWindow<'a> {
    pub fn new(queries: &'a mut HashMap<String, Query>, input_query: &'a mut InputQuery) -> Self {
        Self {
            queries,
            input_query,
        }
    }

    pub fn show(&mut self, ctx: &Context, is_active: &'a mut bool) {
        Window::new("Query window")
            .open(is_active)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Enter a query name: ");
                ui.text_edit_singleline(&mut self.input_query.name);

                ui.label("Enter a SQL query that contains only geometry column: ");

                // from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/code_editor.rs
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        &theme,
                        string,
                        "sql",
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.input_query.sql)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    let submit_button = ui.add(Button::new("Submit query"));

                    if submit_button.clicked() {
                        self.send_query()
                    }
                });

                ui.label("Message:");
                // ui.label(self.connection_status.clone());
            });
    }

    pub fn send_query(&self) {}
}

pub struct Query {
    pub sql: String,
    pub stroke_color: Color32,
    pub stroke_width: f32,
    pub geometries: Vec<SdoGeometry>,
}
