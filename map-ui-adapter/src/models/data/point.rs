use crate::models::dto::point::PointDto;
use derive_new::new;
use sqlx::FromRow;

#[derive(FromRow, new, Debug)]
pub struct PointData {
    pub point_type: String,
    pub name: String,
    pub address: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<PointData> for PointDto {
    fn from(point_data: PointData) -> Self {
        PointDto {
            point_type: point_data.point_type,
            name: point_data.name,
            address: point_data.address,
            description: point_data.description,
            latitude: point_data.latitude,
            longitude: point_data.longitude,
        }
    }
}
