use eframe::App;
use egui::{
    ahash::HashMap, Align, Button, CollapsingHeader, Color32, Frame, Hyperlink, Layout, Response,
    RichText, SidePanel, Ui, Visuals, Window,
};
use egui_plot::Plot;
use serde::{Deserialize, Serialize};

use crate::{
    api::GeometryApi,
    query::{InputQuery, Query, QueryWindow},
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

#[derive(Default)]
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
            input_query: InputQuery::default(),
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

        // From https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/plot_demo.rs#L66-L78
        ui.collapsing("Instructions", |ui| {
            ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
            ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
            if cfg!(target_arch = "wasm32") {
                ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
            } else if cfg!(target_os = "macos") {
                ui.label("Zoom with ctrl / ⌘ + scroll.");
            } else {
                ui.label("Zoom with ctrl + scroll.");
            }
            ui.label("Reset view with double-click.");
        });

        ui.add_space(PADDING);

        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            let btn_emoji: RichText;
            let btn_description: RichText;

            if self.config.is_dark_mode {
                btn_emoji = RichText::new("🔆").text_style(egui::TextStyle::Body);
                btn_description =
                    RichText::new("Switch to light mode").text_style(egui::TextStyle::Body);
            } else {
                btn_emoji = RichText::new("🌙").text_style(egui::TextStyle::Body);
                btn_description =
                    RichText::new("Switch to dark mode").text_style(egui::TextStyle::Body);
            }

            let theme_btn = ui
                .add(Button::new(btn_emoji))
                .on_hover_text(btn_description);

            if theme_btn.clicked() {
                self.config.is_dark_mode = !self.config.is_dark_mode;
                self.save_config();
            }

            let api_url_button = ui.add(Button::new("API configuration"));

            if api_url_button.clicked() {
                self.show_api_config_window = true;
            }

            let geometry_button = ui.add(Button::new("Add geometry"));

            if geometry_button.clicked() {
                self.show_query_window = true;
            }
        });

        ui.add_space(PADDING);

        self.geometry_list(ui);
    }

    pub fn geometry_list(&mut self, ui: &mut Ui) {
        let scroll = egui::ScrollArea::vertical().auto_shrink([false, true]);
        let mut to_remove: Option<String> = None;
        scroll.show(ui, |ui| {
            for (name, query) in self.queries.iter_mut() {
                CollapsingHeader::new(name).show(ui, |ui| {
                    egui::stroke_ui(ui, &mut query.stroke, "Curve Stroke");
                    ui.collapsing("Geometries", |ui| {
                        for geometry in query.geometries.iter_mut() {
                            ui.horizontal_wrapped(|ui| {
                                ui.checkbox(&mut geometry.is_active, geometry.name.clone());
                            });
                        }

                        let toggle_button = ui.add(Button::new("Show all"));
                        if toggle_button.clicked() {
                            for geometry in query.geometries.iter_mut() {
                                geometry.is_active = true;
                            }
                        }
                    });
                    let delete_button = ui.add(Button::new(
                        RichText::new("Delete objects").color(Color32::RED),
                    ));

                    if delete_button.clicked() {
                        to_remove = Some(name.clone());
                    }
                });
            }
        });

        if let Some(name) = to_remove {
            self.queries = std::mem::take(self)
                .queries
                .into_iter()
                .filter(|(n, _q)| *n != name)
                .collect();
        }
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
            for query in self.queries.values() {
                for geometry in query.geometries.iter() {
                    if !geometry.is_active {
                        continue;
                    }

                    if geometry.sdo_geometry.is_polygon() {
                        if let Some(poly) = geometry.sdo_geometry.create_polygon(query.stroke) {
                            plot_ui.polygon(poly.name(geometry.name.clone()))
                        }
                    }

                    if geometry.sdo_geometry.is_circle() {
                        if let Some(circle) = geometry.sdo_geometry.create_circle(query.stroke) {
                            plot_ui.line(circle.name(geometry.name.clone()))
                        }
                    }

                    if geometry.sdo_geometry.is_line() {
                        if let Some(line) = geometry.sdo_geometry.create_line(query.stroke) {
                            plot_ui.line(line.name(geometry.name.clone()))
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
