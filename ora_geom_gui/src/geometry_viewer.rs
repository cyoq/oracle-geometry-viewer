use eframe::App;
use egui::{Button, Hyperlink, Layout, RichText, SidePanel, Ui, Visuals, Window};
use serde::{Deserialize, Serialize};
use tracing::info;

const PADDING: f32 = 15.0;

#[derive(Serialize, Deserialize)]
pub struct GeometryViewerConfig {
    pub is_dark_mode: bool,
    pub api_url: String,
}

impl Default for GeometryViewerConfig {
    fn default() -> Self {
        Self {
            is_dark_mode: Default::default(),
            api_url: String::from("demo"),
        }
    }
}

pub struct GeometryViewer {
    pub config: GeometryViewerConfig,
    pub name: String,
    pub age: u32,
}

impl GeometryViewer {
    pub fn new() -> Self {
        Self {
            config: GeometryViewerConfig::default(),
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
            }

            let api_url_button = ui.add(Button::new("Change API URL"));

            if api_url_button.clicked() {
                info!("Clicked changing API!");
            }

            let geometry_button = ui.add(Button::new("Add geometry"));

            if geometry_button.clicked() {
                info!("Clicked geometry!");
            }
        });
    }

    pub fn render_config(&mut self, ctx: &egui::Context) {
        Window::new("API Configuration").show(ctx, |ui| {
            ui.label("Enter your backend URL for Oracle geometry data retrieval");
            let text_input = ui.text_edit_singleline(&mut self.config.api_url);
            if text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Err(e) = confy::store(
                    "oracle_geometry_viewer",
                    "geometry_viewer_config",
                    GeometryViewerConfig {
                        is_dark_mode: self.config.is_dark_mode,
                        api_url: self.config.api_url.to_string(),
                    },
                ) {
                    tracing::error!("Failed saving app state: {}", e);
                }

                // self.api_key_initialized = true;
                // if let Some(tx) = &self.app_tx {
                //     tx.send(Msg::ApiKeySet(self.config.api_key.to_string()));
                // }

                tracing::error!("api key set");
            }
            tracing::error!("{}", &self.config.api_url);

            ui.label("If you havn't registered for the API_KEY, head over to");
            ui.hyperlink("https://newsapi.org");
        });
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

        self.render_config(ctx);

        SidePanel::new(egui::panel::Side::Left, "side_panel")
            .max_width(BOARD_PANEL_WIDTH)
            .min_width(BOARD_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| self.side_panel(ctx, ui));

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
