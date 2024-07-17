use std::sync::Arc;

use derive_new::new;
use map_ui_adapter::{
    models::dto::point::PointDto, modules::SearvicesModuleExt, services::point::PointService,
};
use map_ui_kernel::models::coordinate::Coordinate;

use crate::models::point::PointCommand;

#[derive(new)]
pub struct PointUseCase<S: SearvicesModuleExt> {
    services: Arc<S>,
}

impl<S> PointUseCase<S>
where
    S: SearvicesModuleExt,
{
    pub async fn get_point(
        &self,
        point_command: Vec<PointCommand>,
    ) -> anyhow::Result<Vec<PointDto>> {
        let coordinates = point_command
            .into_iter()
            .map(|command| command.into())
            .collect::<Vec<Coordinate>>();
        self.services.point_service().get_points(coordinates).await
    }
}
