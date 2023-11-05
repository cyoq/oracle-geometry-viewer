use std::f64::consts::TAU;

use egui::{remap, Color32, Pos2, Stroke};
use egui_plot::{Line, PlotPoints, Polygon};
use serde::Deserialize;

// https://www.reddit.com/r/learnrust/comments/lfw6uy/comment/gmqqhg2/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn collect_array<T, I, const N: usize>(itr: I) -> [T; N]
where
    T: Default + Copy,
    I: IntoIterator<Item = T>,
{
    let mut res = [T::default(); N];
    for (it, elem) in res.iter_mut().zip(itr) {
        *it = elem
    }

    res
}

#[derive(Deserialize, Debug)]
pub struct SdoGeometry {
    pub sdo_gtype: f32,
    pub sdo_srid: Option<f32>,
    pub sdo_point: Option<f32>,
    pub sdo_elem_info: Vec<f32>,
    pub sdo_ordinates: Vec<f64>,
}

impl SdoGeometry {
    fn create_coordinates(&self) -> Vec<[f64; 2]> {
        self.sdo_ordinates
            .chunks(2)
            .map(|c| c.to_owned())
            .map(|c| collect_array(c.into_iter()))
            .collect::<Vec<_>>()
    }

    pub fn is_polygon(&self) -> bool {
        self.sdo_gtype as i32 == 2003 && self.sdo_elem_info == vec![1., 3., 1.]
    }

    pub fn create_polygon(&self, stroke: Stroke) -> Option<Polygon> {
        if !self.is_polygon() {
            return None;
        }

        let coordinates = self.create_coordinates();

        let polygon = Polygon::new(coordinates)
            // egui plot cannot handle concave plots, therefore turning the filling off
            .fill_color(Color32::TRANSPARENT)
            .stroke(stroke);

        Some(polygon)
    }

    pub fn is_circle(&self) -> bool {
        self.sdo_gtype as i32 == 2003 && self.sdo_elem_info == vec![1., 1003., 4.]
    }

    pub fn create_circle(&self, stroke: Stroke) -> Option<Line> {
        if !self.is_circle() {
            return None;
        }

        let radius = (self.sdo_ordinates[5] - self.sdo_ordinates[1]) / 2.;
        let center = Pos2::new(
            (self.sdo_ordinates[1] + radius) as f32,
            (self.sdo_ordinates[2] - radius) as f32,
        );
        let n = 512;
        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = radius;
                [r * t.cos() + center.y as f64, r * t.sin() + center.x as f64]
            })
            .collect();
        Some(Line::new(circle_points).stroke(stroke))
        // .name("circle")
    }

    pub fn is_line(&self) -> bool {
        self.sdo_gtype as i32 == 2002 && self.sdo_elem_info == vec![1., 2., 1.]
    }

    pub fn create_line(&self, stroke: Stroke) -> Option<Line> {
        if !self.is_line() {
            return None;
        }

        let coordinates = self.create_coordinates();
        Some(Line::new(coordinates).stroke(stroke))
    }
}

#[cfg(test)]
mod tests {

    use crate::sdo_geometry::SdoGeometry;

    #[test]
    fn test_is_polygon() {
        let sdo_object = SdoGeometry {
            sdo_gtype: 2003.,
            sdo_srid: None,
            sdo_point: None,
            sdo_elem_info: vec![1., 3., 1.],
            sdo_ordinates: vec![],
        };

        assert!(sdo_object.is_polygon());
    }

    #[test]
    fn test_create_polygon_coordinates() {
        let sdo_object = SdoGeometry {
            sdo_gtype: 2003.,
            sdo_srid: None,
            sdo_point: None,
            sdo_elem_info: vec![1., 3., 1.],
            sdo_ordinates: vec![40., 23., 48., 23., 48., 29., 40., 29., 40., 23.],
        };

        let expected = vec![[40., 23.], [48., 23.], [48., 29.], [40., 29.], [40., 23.]];

        assert_eq!(sdo_object.create_coordinates(), expected);
    }

    #[test]
    fn test_is_line() {
        let sdo_object = SdoGeometry {
            sdo_gtype: 2002.,
            sdo_srid: None,
            sdo_point: None,
            sdo_elem_info: vec![1., 2., 1.],
            sdo_ordinates: vec![],
        };

        assert!(sdo_object.is_line());
    }

    #[test]
    fn test_is_circle() {
        let sdo_object = SdoGeometry {
            sdo_gtype: 2003.,
            sdo_srid: None,
            sdo_point: None,
            sdo_elem_info: vec![1., 1003., 4.],
            sdo_ordinates: vec![],
        };

        assert!(sdo_object.is_circle());
    }
}
