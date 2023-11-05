pub mod geometry_viewer;
pub mod sdo_geometry;

use eframe::egui;
use egui::Visuals;
use geometry_viewer::GeometryViewer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), eframe::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Oracle Geometry Viewer",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            let viewer = GeometryViewer::new();
            Box::new(viewer)
        }),
    )
}
