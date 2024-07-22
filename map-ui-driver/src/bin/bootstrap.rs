use map_ui_driver::{
    modules::Modules,
    startup::{init_app, startup},
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    init_app();

    let modules = Arc::new(Modules::new().await);
    startup(modules).await;
}
