use std::sync::Arc;

use crate::settings::config::AppConfig;
use map_ui_adapter::{
    clients::mysql::MySqlConnection,
    modules::{SearvicesModuleExt, ServicesModule},
};
use map_ui_app::usecase::point::PointUseCase;

pub struct Modules {
    point_usecase: PointUseCase<ServicesModule>,
}

pub trait ModulesExt {
    type ServicesModule: SearvicesModuleExt;

    fn point_usecase(&self) -> &PointUseCase<Self::ServicesModule>;
}

impl ModulesExt for Modules {
    type ServicesModule = ServicesModule;

    fn point_usecase(&self) -> &PointUseCase<Self::ServicesModule> {
        &self.point_usecase
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let config = AppConfig::new().unwrap();
        let mysql_connection = MySqlConnection::new(&config.mysql).await;

        let services_module = Arc::new(ServicesModule::new(Arc::new(mysql_connection)));

        let point_usecase = PointUseCase::new(services_module.clone());

        Self { point_usecase }
    }
}
