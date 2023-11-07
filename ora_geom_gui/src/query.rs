use egui::{ahash::HashMap, Button, Color32, Context, Layout, RichText, Stroke, Window};

use crate::{api::GeometryApi, sdo_geometry::SdoGeometry};

const COLORS: [Color32; 4] = [Color32::RED, Color32::BLUE, Color32::GREEN, Color32::YELLOW];

pub struct InputQuery {
    pub sql: String,
    pub name: String,
    pub message: RichText,
}

impl Default for InputQuery {
    fn default() -> Self {
        Self {
            sql: "-- A very simple example\n\
SELECT\n\
\tGEOMETRY\n\
FROM\n\
\tBUILDINGS\n\
"
            .into(),
            name: "".into(),
            message: RichText::new(""),
        }
    }
}

impl InputQuery {
    pub fn clear(&mut self) {
        self.sql = String::from("");
        self.name = String::from("");
    }
}

pub struct QueryWindow<'a> {
    pub queries: &'a mut HashMap<String, Query>,
    pub input_query: &'a mut InputQuery,
    pub api: &'a GeometryApi,
}

impl<'a> QueryWindow<'a> {
    pub fn new(
        queries: &'a mut HashMap<String, Query>,
        input_query: &'a mut InputQuery,
        api: &'a GeometryApi,
    ) -> Self {
        Self {
            queries,
            input_query,
            api,
        }
    }

    pub fn show(&mut self, ctx: &Context, is_active: &'a mut bool) {
        Window::new("Query window")
            .open(is_active)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Enter a query name: ");
                    ui.text_edit_singleline(&mut self.input_query.name);
                });

                ui.label("Enter a SQL query that contains only geometry column. ");
                ui.label(
                    RichText::new("NOTE: Do not put any semicolons in the query!")
                        .color(Color32::LIGHT_YELLOW),
                );

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
                        self.send_query();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Message:");
                    ui.label(self.input_query.message.clone());
                });
            });
    }

    pub fn send_query(&mut self) {
        if self.input_query.name.is_empty() {
            self.input_query.message =
                RichText::new(String::from("Query must have a name!")).color(Color32::RED);
            return;
        }

        if self.queries.contains_key(&self.input_query.name) {
            self.input_query.message =
                RichText::new(String::from("This name already exists!")).color(Color32::RED);
            return;
        }

        let data = self.api.fetch_geometries(&self.input_query.sql);

        match data {
            Ok(data) => {
                self.queries.insert(
                    self.input_query.name.clone(),
                    Query {
                        sql: self.input_query.sql.clone(),
                        stroke: Stroke::new(1., COLORS[self.queries.len() % 4]),
                        geometries: data
                            .into_iter()
                            .enumerate()
                            .map(|(n, g)| Geometry {
                                name: format!("{}_{}", self.input_query.name.clone(), n),
                                sdo_geometry: g,
                                is_active: true,
                            })
                            .collect::<Vec<_>>(),
                    },
                );

                self.input_query.message =
                    RichText::new("Successfully fetched data").color(Color32::GREEN);
            }
            Err(e) => {
                self.input_query.message =
                    RichText::new(format!("An error occured while sending the query: {e}"))
                        .color(Color32::RED);
            }
        }
    }
}

pub struct Geometry {
    pub name: String,
    pub sdo_geometry: SdoGeometry,
    pub is_active: bool,
}

pub struct Query {
    pub sql: String,
    pub stroke: Stroke,
    pub geometries: Vec<Geometry>,
}
