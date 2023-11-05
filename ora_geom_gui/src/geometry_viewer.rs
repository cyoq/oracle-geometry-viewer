use eframe::App;
use egui::{
    ahash::HashMap, Align, Button, Color32, Frame, Hyperlink, Layout, Response, RichText,
    SidePanel, Stroke, Ui, Visuals, Window,
};
use egui_plot::{Line, Plot, Polygon};
use serde::{Deserialize, Serialize};

use crate::{
    api::GeometryApi,
    query::{Query, QueryWindow},
    sdo_geometry::SdoGeometry,
};

const PADDING: f32 = 15.0;
const CONFY_APP: &'static str = "oracle_geometry_viewer";
const CONFY_CONFIG: &'static str = "geometry_viewer_config";

#[derive(Serialize, Deserialize)]
pub struct GeometryViewerConfig {
    pub is_dark_mode: bool,
    pub api: GeometryApi,
}

impl Default for GeometryViewerConfig {
    fn default() -> Self {
        Self {
            is_dark_mode: Default::default(),
            api: GeometryApi::new(),
        }
    }
}

pub struct InputQuery {
    pub sql: String,
    pub name: String,
    pub message: RichText,
}

impl InputQuery {
    pub fn new() -> Self {
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

    pub fn clear(&mut self) {
        self.sql = String::from("");
        self.name = String::from("");
    }
}

pub struct GeometryViewer {
    pub config: GeometryViewerConfig,
    pub show_api_config_window: bool,
    pub show_query_window: bool,
    pub connection_status: RichText,
    pub queries: HashMap<String, Query>,
    pub input_query: InputQuery,
}

impl GeometryViewer {
    pub fn new() -> Self {
        let config = confy::load(CONFY_APP, CONFY_CONFIG).unwrap_or_default();

        Self {
            config,
            show_api_config_window: true,
            show_query_window: false,
            connection_status: RichText::new(""),
            queries: HashMap::default(),
            input_query: InputQuery::new(),
        }
    }

    pub fn side_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Oracle Geometry Viewer");
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            ui.add_space(PADDING);
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Made by cyoq").text_style(egui::TextStyle::Monospace),
                "https://github.com/cyoq",
            ));
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Made with egui").text_style(egui::TextStyle::Monospace),
                "https://github.com/emilk/egui",
            ));
            ui.add_space(PADDING);
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            ui.heading("Geometries");
        });

        ui.add_space(PADDING);

        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            let btn_emoji: RichText;
            let btn_description: RichText;

            if self.config.is_dark_mode {
                btn_emoji = RichText::new("🔆").text_style(egui::TextStyle::Body);
                btn_description =
                    RichText::new("Swittch to light mode").text_style(egui::TextStyle::Body);
            } else {
                btn_emoji = RichText::new("🌙").text_style(egui::TextStyle::Body);
                btn_description =
                    RichText::new("Swittch to dark mode").text_style(egui::TextStyle::Body);
            }

            let theme_btn = ui
                .add(Button::new(btn_emoji))
                .on_hover_text(btn_description);

            if theme_btn.clicked() {
                self.config.is_dark_mode = !self.config.is_dark_mode;
                self.save_config();
            }

            let api_url_button = ui.add(Button::new("Change API URL"));

            if api_url_button.clicked() {
                self.show_api_config_window = true;
            }

            let geometry_button = ui.add(Button::new("Add geometry"));

            if geometry_button.clicked() {
                self.show_query_window = true;
            }
        });
    }

    pub fn save_config(&mut self) {
        if let Err(e) = confy::store(
            CONFY_APP,
            CONFY_CONFIG,
            GeometryViewerConfig {
                is_dark_mode: self.config.is_dark_mode,
                api: self.config.api.clone(),
            },
        ) {
            tracing::error!("Failed saving app state: {}", e);
        }
    }

    pub fn render_api_config(&mut self, ctx: &egui::Context) {
        Window::new("API Configuration")
            .collapsible(true)
            .show(ctx, |ui| {
                ui.label("Enter your backend URL for Oracle geometry data retrieval");
                let text_input = ui.text_edit_singleline(&mut self.config.api.api_url);

                let pressed_enter =
                    text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if pressed_enter {
                    self.save_config();
                    self.show_api_config_window = false;
                }

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let submit_button = ui.add(Button::new("Create connection"));

                    if submit_button.clicked() {
                        self.save_config();
                        self.connection_status = self.config.api.connection_status();
                        self.show_api_config_window = false;
                    }

                    let test_button = ui.add(Button::new("Test connection"));

                    if test_button.clicked() {
                        self.connection_status = self.config.api.connection_status();
                    }
                });

                ui.label("Connection status:");
                ui.label(self.connection_status.clone());
            });
    }

    pub fn geometry_content(&mut self, ui: &mut Ui) -> Response {
        let plot = Plot::new("oracle_geometry").y_axis_width(3).data_aspect(1.);

        plot.show(ui, |plot_ui| {
            for (name, query) in self.queries.iter() {
                for (num, geometry) in query.geometries.iter().enumerate() {
                    if geometry.is_polygon() {
                        if let Some(poly) =
                            geometry.create_polygon(Stroke::new(1., Color32::LIGHT_RED))
                        {
                            plot_ui.polygon(poly.name(format!("{name}_{num}")))
                        }
                    }

                    if geometry.is_circle() {
                        if let Some(circle) =
                            geometry.create_circle(Stroke::new(1., Color32::LIGHT_BLUE))
                        {
                            plot_ui.line(circle.name(format!("{name}_{num}")))
                        }
                    }

                    if geometry.is_line() {
                        if let Some(line) =
                            geometry.create_line(Stroke::new(1., Color32::LIGHT_BLUE))
                        {
                            plot_ui.line(line.name(format!("{name}_{num}")))
                        }
                    }
                }
            }
            plot_ui.pointer_coordinate();
            plot_ui.pointer_coordinate_drag_delta();
            plot_ui.plot_bounds();
            plot_ui.response().hovered();
        })
        .response
    }
}

const BOARD_PANEL_WIDTH: f32 = 300.0;

impl App for GeometryViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.config.is_dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.show_api_config_window {
            self.render_api_config(ctx);
        }

        if self.show_query_window {
            QueryWindow::new(&mut self.queries, &mut self.input_query, &self.config.api)
                .show(ctx, &mut self.show_query_window);
        }

        SidePanel::new(egui::panel::Side::Left, "side_panel")
            .max_width(BOARD_PANEL_WIDTH)
            .min_width(BOARD_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| self.side_panel(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| self.geometry_content(ui));
        });
    }
}
