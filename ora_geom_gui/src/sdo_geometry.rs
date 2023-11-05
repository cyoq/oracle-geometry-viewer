pub struct SdoGeometry {
    pub sdo_gtype: f32,
    pub sdo_srid: Option<f32>,
    pub sdo_point: Option<f32>,
    pub sdo_elem_info: Vec<f32>,
    pub sdo_ordinates: Vec<f32>,
}
