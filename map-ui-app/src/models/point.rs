use derive_new::new;
use map_ui_kernel::models::coordinate::Coordinate;

#[derive(new)]
pub struct PointCommand {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<PointCommand> for Coordinate {
    fn from(command: PointCommand) -> Self {
        Self {
            latitude: command.latitude,
            longitude: command.longitude,
        }
    }
}
