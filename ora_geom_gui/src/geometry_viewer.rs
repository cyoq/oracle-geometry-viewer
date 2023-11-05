use std::f64::consts::TAU;

use eframe::App;
use egui::{
    emath, remap, Align, Button, Color32, Frame, Hyperlink, Layout, Pos2, Rect, Response, RichText,
    Sense, SidePanel, Stroke, Ui, Vec2, Visuals, Window,
};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, PlotResponse, Polygon};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::sdo_geometry::SdoGeometry;

const PADDING: f32 = 15.0;
const CONFY_APP: &'static str = "oracle_geometry_viewer";
const CONFY_CONFIG: &'static str = "geometry_viewer_config";

pub enum ApiHealth {
    Ok,
    Error(String),
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiConnection {
    pub api_url: String,
}

impl ApiConnection {
    pub fn new() -> Self {
        Self {
            api_url: String::from("demo"),
        }
    }

    pub fn test_connection(&self) -> ApiHealth {
        ApiHealth::Ok
    }

    pub fn connection_status(&self) -> RichText {
        match self.test_connection() {
            ApiHealth::Ok => RichText::new("OK!").color(Color32::GREEN),
            ApiHealth::Error(e) => RichText::new(format!("Error: {e}")).color(Color32::RED),
            ApiHealth::Unknown => RichText::new(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeometryViewerConfig {
    pub is_dark_mode: bool,
    pub api: ApiConnection,
}

impl Default for GeometryViewerConfig {
    fn default() -> Self {
        Self {
            is_dark_mode: Default::default(),
            api: ApiConnection::new(),
        }
    }
}

pub struct GeometryViewer {
    pub config: GeometryViewerConfig,
    pub show_api_config_window: bool,
    pub connection_status: RichText,
    pub name: String,
    pub age: u32,
}

impl GeometryViewer {
    pub fn new() -> Self {
        let config = confy::load(CONFY_APP, CONFY_CONFIG).unwrap_or_default();

        Self {
            config,
            show_api_config_window: true,
            connection_status: RichText::new(""),
            name: String::from("Arthur"),
            age: 42,
        }
    }

    pub fn side_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) {
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
                info!("Clicked geometry!");
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
        let sdo_object = SdoGeometry {
            sdo_gtype: 2003.,
            sdo_srid: None,
            sdo_point: None,
            sdo_elem_info: vec![1., 3., 1.],
            sdo_ordinates: vec![40., 23., 48., 23., 48., 29., 40., 29., 40., 23.],
        };

        let plot = Plot::new("interaction_demo")
            .y_axis_width(3)
            .data_aspect(1.);

        let polygon = Polygon::new(vec![
            [40., 23.],
            [48., 23.],
            [48., 29.],
            [40., 29.],
            [40., 23.],
        ])
        .fill_color(Color32::TRANSPARENT);

        let polygon2 = Polygon::new(vec![
            [1., 10.],
            [3., 10.],
            [3., 12.],
            [7., 12.],
            [7., 10.],
            [9., 10.],
            [9., 19.],
            [1., 19.],
            [1., 10.],
        ])
        .fill_color(Color32::TRANSPARENT)
        .stroke(Stroke::new(3.0, Color32::GREEN));

        let polygons = [polygon, polygon2];

        plot.show(ui, |plot_ui| {
            for polygon in polygons {
                plot_ui.polygon(polygon.name("Concave"))
            }
            plot_ui.line(self.circle());
            plot_ui.pointer_coordinate();
            plot_ui.pointer_coordinate_drag_delta();
            plot_ui.plot_bounds();
            plot_ui.response().hovered();
        })
        .response
    }

    fn circle(&self) -> Line {
        let n = 512;
        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = 3.;
                [r * t.cos() + 4. as f64, r * t.sin() + 4. as f64]
            })
            .collect();
        Line::new(circle_points)
            .color(Color32::from_rgb(100, 200, 100))
            .name("circle")
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

        SidePanel::new(egui::panel::Side::Left, "side_panel")
            .max_width(BOARD_PANEL_WIDTH)
            .min_width(BOARD_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| self.side_panel(ctx, ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| self.geometry_content(ui));
        });
    }
}
