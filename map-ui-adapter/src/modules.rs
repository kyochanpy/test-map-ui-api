use std::sync::Arc;

use map_ui_kernel::models::coordinate::Coordinate;

use crate::services::point::PointService;
use crate::{clients::mysql::MySqlConnection, services::ServiceImpl};

pub struct ServicesModule {
    pub point_service: ServiceImpl<Coordinate>,
}

pub trait SearvicesModuleExt {
    type PointServ: PointService;
    fn point_service(&self) -> &Self::PointServ;
}

impl SearvicesModuleExt for ServicesModule {
    type PointServ = ServiceImpl<Coordinate>;
    fn point_service(&self) -> &Self::PointServ {
        &self.point_service
    }
}

impl ServicesModule {
    pub fn new(mysql_connection: Arc<MySqlConnection>) -> Self {
        Self {
            point_service: ServiceImpl::new(mysql_connection),
        }
    }
}
