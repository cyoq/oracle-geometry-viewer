use egui::Color32;

use crate::sdo_geometry::SdoGeometry;

pub struct Query {
    pub sql: String,
    pub stroke_color: Color32,
    pub stroke_width: f32,
    pub geometries: Vec<SdoGeometry>,
}
