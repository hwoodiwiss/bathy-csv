use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct BathySurface {
    pub points: Vec<BathyPoint>,
    bounds: Option<Rect>,
}

#[derive(Debug, Deserialize)]
pub struct BathyPointRaw {
    #[serde(rename(deserialize = "Lat (DD)"))]
    pub lat: f32,
    #[serde(rename(deserialize = "Long (DD)"))]
    pub long: f32,
    #[serde(rename(deserialize = "Depth"))]
    pub depth: f32,
}

#[derive(Debug)]
pub struct BathyPoint {
    pub lat: f32,
    pub long: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}

impl BathyPoint {
    pub fn from_raw(raw_point: &BathyPointRaw) -> BathyPoint {
        BathyPoint {
            lat: 111000.0 * raw_point.lat,
            long: 111000.0 * raw_point.long,
            depth: -raw_point.depth,
        }
    }
}

impl BathySurface {
    pub fn from_path(path: &str) -> Result<BathySurface, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;

        let mut points = Vec::<BathyPoint>::new();
        for result in reader.deserialize() {
            let metric_point = BathyPoint::from_raw(&result?);
            points.push(metric_point)
        }

        Ok(BathySurface {
            points,
            bounds: None,
        })
    }

    /// Returns the rectangular bounds of the
    pub fn get_bounds(&mut self) -> Rect {
        if let Some(stored_bounds) = self.bounds {
            stored_bounds
        } else {
            self.bounds = Some(self.generate_bounds());
            self.get_bounds()
        }
    }

    fn generate_bounds(&self) -> Rect {
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for point in &self.points {
            min_x = (*point).long.min(min_x);
            max_x = (*point).long.max(max_x);
            min_y = (*point).lat.min(min_y);
            max_y = (*point).lat.max(max_y);
        }

        Rect {
            x0: min_x,
            x1: max_x,
            y0: min_y,
            y1: max_y,
        }
    }
}
