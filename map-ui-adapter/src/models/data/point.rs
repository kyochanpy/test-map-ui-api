use derive_new::new;
use sqlx::FromRow;

use crate::models::dto::point::PointDto;

#[derive(FromRow, new)]
pub struct PointData {
    pub point_type: String,
    pub name: String,
    pub address: String,
    pub description: String,
}

impl From<PointData> for PointDto {
    fn from(point_data: PointData) -> Self {
        PointDto {
            point_type: point_data.point_type,
            name: point_data.name,
            address: point_data.address,
            description: point_data.description,
        }
    }
}
