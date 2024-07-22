use crate::models::{data::point::PointData, dto::point::PointDto};

use super::ServiceImpl;
use anyhow::{Ok, Result};
use async_trait::async_trait;

use map_ui_kernel::models::coordinate::Coordinate;

#[async_trait]
pub trait PointService {
    async fn get_points(&self, coordinates: Vec<Coordinate>) -> Result<Vec<PointDto>>;
}

#[async_trait]
impl PointService for ServiceImpl<Coordinate> {
    async fn get_points(&self, coordinates: Vec<Coordinate>) -> Result<Vec<PointDto>> {
        let polygon = coordinates
            .iter()
            .map(|coordinate| format!("{} {}", coordinate.longitude, coordinate.latitude))
            .collect::<Vec<String>>()
            .join(", ");
        let polygon = format!("POLYGON(({}))", polygon);
        let query = "
            SELECT
                point_type, name, address, description, latitude, longitude
            FROM
                test_map_ui.master
            WHERE
                ST_Contains(ST_GeomFromText(?), location)
        ";

        let points = sqlx::query_as::<_, PointData>(query)
            .bind(polygon)
            .fetch_all(&*self.mysql_connection.pool)
            .await?;

        let mut points_dto = Vec::new();
        for point in points {
            points_dto.push(PointDto::from(point));
        }
        Ok(points_dto)
    }
}
