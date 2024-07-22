use derive_new::new;
use map_ui_adapter::models::dto::point::PointDto;
use map_ui_app::models::point::PointCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct JsonPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<JsonPoint> for PointCommand {
    fn from(value: JsonPoint) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}

#[derive(Debug, new, Serialize)]
pub struct Point {
    pub point_type: String,
    pub name: String,
    pub address: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<PointDto> for Point {
    fn from(value: PointDto) -> Self {
        Self {
            point_type: value.point_type,
            name: value.name,
            address: value.address,
            description: value.description,
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}
